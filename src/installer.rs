use dirs::home_dir;
use eyre::Result;
use flate2::read::GzDecoder;
use tar::Archive;
use crate::{chromedriver::Chromedriver, geckodriver::Geckodriver, Driver, DriverFetcher};

use std::path::PathBuf;

/// Downloads, unarchives and moves the driver executable to $HOME
pub fn install_latest(driver: Driver) -> Result<PathBuf> {
    let download_url = match driver {
        Driver::Gecko => {
            let version = Geckodriver::new().latest_version()?;
            Geckodriver::new().direct_download_url(&version)?
        }
        Driver::Chrome => {
            let version = Geckodriver::new().latest_version()?;
            Chromedriver::new().direct_download_url(&version)?
        }
    };
    let resp = reqwest::blocking::get(download_url.clone())?;
    let content = &resp.bytes()?;

    let fname = download_url
        .path_segments()
        .and_then(|s| s.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("tmp.bin");

    let target_dir = home_dir().unwrap().join(".webdrivers");
    std::fs::create_dir_all(&target_dir)?;

    decompress(fname, content, target_dir.clone())?;

    println!("stored in {:?}", target_dir);
    Ok(target_dir)
}

fn decompress(filename: &str, bytes: &[u8], target_dir: PathBuf) -> Result<PathBuf> {
    match filename {
        name if name.ends_with("tar.gz") => {
            let tar = GzDecoder::new(std::io::Cursor::new(bytes));
            let mut archive = Archive::new(tar);

            let driver_executable = archive.entries()?.filter_map(Result::ok).filter(|e| {
                let filename = e.path().unwrap();
                println!("filename: {:?}", filename);
                filename.as_os_str() == "geckodriver"
            });

            for mut exec in driver_executable {
                let final_path = target_dir.join(exec.path()?);
                exec.unpack(final_path)?;
            }
        }
        ext => panic!("No support for unarchiving {}, yet", ext),
    }
    Ok(target_dir)
}

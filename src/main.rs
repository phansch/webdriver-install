mod geckodriver;
use webdriver_install::DriverFetcher;
use tar::Archive;
use flate2::read::GzDecoder;
use url::Url;
use dirs::home_dir;
use std::path::PathBuf;

use eyre::Result;

fn main() -> Result<()> {
    let version = geckodriver::Geckodriver::new().latest_version()?;
    let download_url = geckodriver::Geckodriver::new().direct_download_url(&version)?;
    println!("point release: {}", version);
    println!("direct_download_url: {}", &download_url);

    // NOTE: when tmp_dir goes out of scope, the directory will be removed automatically
    let _unarchived_file_path = install(download_url)?;

    Ok(())
}

// Downloads, unarchives and moves the driver executable to $HOME
fn install(download_url: Url) -> Result<PathBuf> {
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
        ext => panic!("No support for unarchiving {}, yet", ext)
    }
    Ok(target_dir)
}

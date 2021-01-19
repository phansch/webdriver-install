use crate::{chromedriver::Chromedriver, geckodriver::Geckodriver, Driver, DriverFetcher};
use dirs::home_dir;
use eyre::{ensure, eyre, Result};
use flate2::read::GzDecoder;
use tar::Archive;
use tracing::debug;

use std::fs::File;
use std::io::{Cursor, Read};
use std::path::PathBuf;

static DRIVER_EXECUTABLES: &[&'static str] = &["geckodriver", "chromedriver"];

/// Downloads and unarchives the driver executable to $HOME/.webdrivers
pub fn install(driver: Driver) -> Result<PathBuf> {
    let target_dir = home_dir().unwrap().join(".webdrivers");
    std::fs::create_dir_all(&target_dir)?;
    install_into(driver, target_dir)
}

/// Downloads and unarchives the driver executable into the specified `target_dir`
pub fn install_into(driver: Driver, target_dir: PathBuf) -> Result<PathBuf> {
    ensure!(target_dir.is_dir(), "`target_dir` must be a directory.");

    let download_url = match driver {
        Driver::Gecko => {
            let version = Geckodriver::new().latest_version()?;
            Geckodriver::new().direct_download_url(&version)?
        }
        Driver::Chrome => {
            let version = Chromedriver::new().latest_version()?;
            Chromedriver::new().direct_download_url(&version)?
        }
    };
    let resp = reqwest::blocking::get(download_url.clone())?;
    let archive_content = &resp.bytes()?;

    let archive_filename = download_url
        .path_segments()
        .and_then(|s| s.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("tmp.bin");

    let executable_path = decompress(archive_filename, archive_content, target_dir.clone())?;

    // Make sure the extracted file will be executable
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        use std::fs;
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&executable_path, fs::Permissions::from_mode(0o775)).unwrap();
    }

    debug!("stored at {:?}", executable_path);
    Ok(executable_path)
}

fn decompress(archive_filename: &str, bytes: &[u8], target_dir: PathBuf) -> Result<PathBuf> {
    match archive_filename {
        name if name.ends_with("tar.gz") => {
            let tar = GzDecoder::new(Cursor::new(bytes));
            let mut archive = Archive::new(tar);

            let driver_executable = archive.entries()?.filter_map(Result::ok).filter(|e| {
                let filename = e.path().unwrap();
                debug!("filename: {:?}", filename);
                DRIVER_EXECUTABLES.contains(&filename.as_os_str().to_str().unwrap())
            });

            for mut exec in driver_executable {
                let final_path = target_dir.join(exec.path()?);
                exec.unpack(&final_path)?;

                return Ok(final_path);
            }
        }
        name if name.ends_with("zip") => {
            debug!("zip file name: {}", name);
            let mut zip = zip::ZipArchive::new(Cursor::new(bytes))?;

            let mut zip_bytes: Vec<u8> = vec![];
            let mut filename: Option<String> = None;
            for i in 0..zip.len() {
                let mut file = zip.by_index(i)?;
                if DRIVER_EXECUTABLES.contains(&file.name()) {
                    filename = Some(file.name().to_string());
                    file.read_to_end(&mut zip_bytes)?;
                    break;
                }
            }
            if let Some(name) = filename {
                debug!("saving zip file: {}", name);
                let executable_path = target_dir.join(name);
                let mut f = File::create(&executable_path)?;
                std::io::copy(&mut zip_bytes.as_slice(), &mut f)?;

                return Ok(executable_path);
            }
        }

        ext => return Err(eyre!("No support for unarchiving {}, yet", ext)),
    }
    Err(eyre!("This installer code should be unreachable!"))
}

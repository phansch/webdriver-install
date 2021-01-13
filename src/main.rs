mod geckodriver;
use webdriver_install::DriverFetcher;
use tempfile::Builder;
use tar::Archive;
use flate2::read::GzDecoder;
use url::Url;
use std::path::PathBuf;

use eyre::Result;

fn main() -> Result<()> {
    let version = geckodriver::Geckodriver::new().latest_version()?;
    let download_url = geckodriver::Geckodriver::new().direct_download_url(&version)?;
    println!("point release: {}", version);
    println!("direct_download_url: {}", &download_url);

    // NOTE: when tmp_dir goes out of scope, the directory will be removed automatically
    let tmp_dir = Builder::new().prefix("webdriver-install").tempdir()?;
    let tmp_path = tmp_dir.path();
    let _unarchived_file_path = download_to_tmp(tmp_path.to_path_buf(), download_url)?;

    Ok(())
}

fn download_to_tmp(tmp_dir: PathBuf, download_url: Url) -> Result<PathBuf> {
    let resp = reqwest::blocking::get(download_url.clone())?;
    let content = &resp.bytes()?;

    let fname = download_url
        .path_segments()
        .and_then(|s| s.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("tmp.bin");
    let target = tmp_dir.join(&fname);

    decompress(fname, content, target.clone())?;


    println!("stored in {:?}", target);
    Ok(target)
}

fn decompress(filename: &str, bytes: &[u8], target: PathBuf) -> Result<PathBuf> {
    match filename {
        name if name.ends_with("tar.gz") => {
            let tar = GzDecoder::new(std::io::Cursor::new(bytes));
            let mut archive = Archive::new(tar);

            archive.unpack(&target)?;
        }
        ext => panic!("No support for unarchiving {}, yet", ext)
    }
    Ok(target)
}

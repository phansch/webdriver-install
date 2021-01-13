mod geckodriver;
use webdriver_install::DriverFetcher;

use eyre::Result;

fn main() -> Result<()> {
    let version = geckodriver::Geckodriver::new().latest_version()?;
    println!("point release: {}", version);
    println!("direct_download_url: {}", geckodriver::Geckodriver::new().direct_download_url(&version)?);
    Ok(())
}

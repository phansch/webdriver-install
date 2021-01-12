mod geckodriver;
use webdriver_install::DriverFetcher;

use eyre::Result;

fn main() -> Result<()> {
    let version = geckodriver::Geckodriver::latest_version()?;
    println!("point release: {}", version);
    Ok(())
}

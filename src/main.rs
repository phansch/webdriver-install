use eyre::Result;
use webdriver_install::chromedriver;
// use webdriver_install::geckodriver;
// use webdriver_install::installer;
// use webdriver_install::DriverFetcher;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // let version = geckodriver::Geckodriver::new().latest_version()?;
    // let download_url = geckodriver::Geckodriver::new().direct_download_url(&version)?;
    // println!("point release: {}", version);
    // println!("direct_download_url: {}", &download_url);

    // let _unarchived_file_path = installer::install(download_url)?;
    match chromedriver::Version::find() {
        Ok(loc) => println!("Chrome found: {:?}", loc.build_version()),
        Err(e) => println!("Error: {:#}", e),
    }

    Ok(())
}


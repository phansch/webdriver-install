use eyre::Result;
use webdriver_install::DriverFetcher;

pub struct Geckodriver;

impl DriverFetcher for Geckodriver {
    const BASE_URL: &'static str = "https://github.com/mozilla/geckodriver/releases";

    /// Returns the latest version of the driver
    /// (Does not download the driver)
    fn latest_version() -> Result<String> {
        let latest_release_url = format!("{}/latest", Self::BASE_URL);
        let resp = reqwest::blocking::get(&latest_release_url)?;
        let url = resp.url();
        Ok(url.path_segments().unwrap().last().unwrap().to_string())
    }
}

impl Geckodriver {
    // fn newest_point_release(version: &str) -> Result<String> {
    //     Ok(reqwest::blocking::get(BASE_URL)?.text()?)
    // }
}

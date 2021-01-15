use eyre::Result;
use url::Url;

pub mod chromedriver;
pub mod geckodriver;
pub mod installer;

pub trait DriverFetcher {
    const BASE_URL: &'static str;

    fn latest_version(&self) -> Result<String>;

    fn direct_download_url(&self, version: &str) -> Result<Url>;
}

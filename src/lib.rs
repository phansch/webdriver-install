pub mod chromedriver;
pub mod geckodriver;
pub mod installer;

pub use installer::install_latest;
use eyre::Result;
use url::Url;

pub enum Driver {
    Chrome,
    Gecko,
}

pub trait DriverFetcher {
    const BASE_URL: &'static str;

    fn latest_version(&self) -> Result<String>;

    fn direct_download_url(&self, version: &str) -> Result<Url>;
}

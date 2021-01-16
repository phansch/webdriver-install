pub mod chromedriver;
pub mod geckodriver;
pub mod installer;

// Re-export these so that users don't have to type something like
//
//     webdriver_installer::installer::install(...)
use eyre::Result;
pub use installer::{install, install_into};
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

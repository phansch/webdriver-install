use eyre::Result;
use url::Url;

pub trait DriverFetcher {
    const BASE_URL: &'static str;

    fn latest_version(&self) -> Result<String>;

    fn direct_download_url(&self, version: &str) -> Result<Url>;
}

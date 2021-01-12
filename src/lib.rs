use eyre::Result;

pub trait DriverFetcher {
    const BASE_URL: &'static str;

    fn latest_version() -> Result<String>;
}

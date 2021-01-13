use eyre::{eyre, Result};
use url::Url;
use webdriver_install::DriverFetcher;

pub struct Geckodriver;

impl DriverFetcher for Geckodriver {
    const BASE_URL: &'static str = "https://github.com/mozilla/geckodriver/releases";

    /// Returns the latest version of the driver
    /// (Does not download the driver)
    fn latest_version(&self) -> Result<String> {
        let latest_release_url = format!("{}/latest", Self::BASE_URL);
        let resp = reqwest::blocking::get(&latest_release_url)?;
        let url = resp.url();
        Ok(url.path_segments().unwrap().last().unwrap().to_string())
    }

    fn direct_download_url(&self, version: &str) -> Result<Url> {
        Ok(
            Url::parse(&format!(
                "{}/download/{version}/geckodriver-{version}-{platform}",
                Self::BASE_URL,
                version=version,
                platform=Self::platform()?
            ))?
        )
    }
}

impl Geckodriver {
    pub fn new() -> Self {
        Geckodriver {}
    }

    pub fn platform() -> Result<String> {
        match sys_info::os_type()?.as_str() {
            "Linux" => Ok(format!("linux{}.tar.gz", Self::pointer_width())),
            "Darwin" => Ok(String::from("macos.tar.gz")),
            "Windows" => Ok(format!("win{}.zip", Self::pointer_width())),
            other => Err(eyre!("webdriver-install doesn't support '{}' currently", other))
        }
    }

    pub const fn pointer_width() -> usize {
        #[cfg(target_pointer_width = "32")]
        {
            32
        }
        #[cfg(target_pointer_width = "64")]
        {
            64
        }
    }
}

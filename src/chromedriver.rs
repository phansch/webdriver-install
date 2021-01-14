use eyre::{eyre, Result};
use url::Url;
use webdriver_install::DriverFetcher;

use std::path::PathBuf;

pub struct Chromedriver;

impl DriverFetcher for Chromedriver {
    const BASE_URL: &'static str = "https://chromedriver.storage.googleapis.com";

    fn latest_version(&self) -> Result<String> {
        // TODO:
        //
        // 1. Figure out the current Chrome version
        // 2. Download and read the LATEST_RELEASE_<chrome_build_version> file
        // 3. Done?
        Ok("TODO".into())
    }

    fn direct_download_url(&self, version: &str) -> Result<Url> {
        Ok(Url::parse(&format!(
            "{}/{version}/chromedriver_{platform}.zip",
            Self::BASE_URL,
            version = version,
            platform = Self::platform()?
        ))?)
    }
}

impl Chromedriver {
    pub fn new() -> Self {
        Self {}
    }

    fn platform() -> Result<String> {
        Ok("".into())
    }
}

pub struct ChromeVersion {
    major: i16,
    minor: i16,
    build: i16,
    patch: i16,
}

pub struct ChromeLocation {}

static CHROME_DIRS: &[&'static str] = &[
    "/usr/local/sbin",
    "/usr/local/bin",
    "/usr/sbin",
    "/usr/bin",
    "/sbin",
    "/bin",
    "/opt/google/chrome",
];
static CHROME_FILES: &[&'static str] = &["google-chrome", "chrome", "chromium", "chromium-browser"];

impl ChromeVersion {
    /// Returns the version of the currently installed Chrome/Chromium browser
    pub fn version() -> Result<Self> {
        #[cfg(target_os = "linux")]
        Self::linux_version()
    }

    fn linux_version() -> Result<Self> {
        // TODO: WSL
        Ok(Self {
            major: 1,
            minor: 2,
            build: 2222,
            patch: 5,
        })
    }
}

impl ChromeLocation {
    /// Returns the location of the currently installed Chrome/Chromium browser
    pub fn location() -> Result<PathBuf> {
        #[cfg(target_os = "linux")]
        Self::linux_location()
    }

    fn linux_location() -> Result<PathBuf> {
        // TODO: WSL
        for dir in CHROME_DIRS.into_iter().map(PathBuf::from) {
            for file in CHROME_FILES {
                let path = dir.join(file);
                if path.exists() {
                    return Ok(path);
                }
            }
        }
        Err(eyre!("Unable to find chrome executable"))
    }
}

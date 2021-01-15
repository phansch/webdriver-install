/// This module manages version selection of the `chromedriver`,
/// based on the installed browser version.
///
/// See https://chromedriver.chromium.org/downloads/version-selection
use eyre::{ensure, eyre, Result};
use regex::Regex;
use tracing::debug;
use url::Url;

use crate::DriverFetcher;

use std::path::PathBuf;
use std::process::{Command, Stdio};

pub struct Chromedriver;

impl DriverFetcher for Chromedriver {
    const BASE_URL: &'static str = "https://chromedriver.storage.googleapis.com";

    /// Returns the latest version of the driver
    fn latest_version(&self) -> Result<String> {
        let latest_release_url = format!("{}/LATEST_RELEASE_{}", Self::BASE_URL, Version::find()?.build_version());
        debug!("latest_release_url: {}", latest_release_url);
        let resp = reqwest::blocking::get(&latest_release_url)?;
        Ok(resp.text()?)
    }

    /// Returns the download url for the driver executable
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

    /// Returns the platform part to be used in the download URL
    ///
    /// The `match` is based on the file contents of, for example
    /// https://chromedriver.storage.googleapis.com/index.html?path=72.0.3626.69/
    ///
    /// If future chromedriver releases have multiple pointer widths per platform,
    /// we have to change this to work like `Geckodriver::platform`.
    fn platform() -> Result<String> {
        match sys_info::os_type()?.as_str() {
            "Linux" => Ok(String::from("linux64.zip")),
            "Darwin" => Ok(String::from("mac64.zip")),
            "Windows" => Ok(String::from("win32.zip")),
            other => Err(eyre!(
                "webdriver-install doesn't support '{}' currently",
                other
            )),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Version {
    major: i16,
    minor: i16,
    build: i16,
    patch: i16,
}

struct Location {}

static LINUX_CHROME_DIRS: &[&'static str] = &[
    "/usr/local/sbin",
    "/usr/local/bin",
    "/usr/sbin",
    "/usr/bin",
    "/sbin",
    "/bin",
    "/opt/google/chrome",
];
static LINUX_CHROME_FILES: &[&'static str] =
    &["google-chrome", "chrome", "chromium", "chromium-browser"];

impl Version {
    /// Returns the version of the currently installed Chrome/Chromium browser
    pub fn find() -> Result<Self> {
        #[cfg(target_os = "linux")]
        Self::linux_version()
    }

    /// Returns major.minor.build.patch
    pub fn full_version(&self) -> String {
        format!(
            "{}.{}.{}.{}",
            self.major, self.minor, self.build, self.patch
        )
    }

    /// Returns major.minor.build
    pub fn build_version(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.build)
    }

    fn linux_version() -> Result<Self> {
        // TODO: WSL?
        let output = Command::new(Location::location()?)
            .arg("--version")
            .stdout(Stdio::piped())
            .output()?
            .stdout;

        let output = String::from_utf8(output)?;
        debug!("Chrome --version output: {}", output);

        Ok(Self::version_from_output(&output)?)
    }

    fn version_from_output(output: &str) -> Result<Self> {
        let version_pattern = Regex::new(r"\d+\.\d+\.\d+\.\d+")?;
        let version = version_pattern
            .captures(&output)
            .ok_or(eyre!("regex: Could not match Chrome version string"))?
            .get(0)
            .map_or("", |m| m.as_str());
        let parts: Vec<i16> = version
            .split(".")
            .map(|i| i.parse::<i16>().unwrap())
            .collect();

        ensure!(
            parts.len() == 4,
            "Expected Chrome version to have 4 parts, but had {}: {}",
            parts.len(),
            version
        );

        Ok(Self {
            major: parts[0],
            minor: parts[1],
            build: parts[2],
            patch: parts[3],
        })
    }
}

impl Location {
    /// Returns the location of the currently installed Chrome/Chromium browser
    pub fn location() -> Result<PathBuf> {
        #[cfg(target_os = "linux")]
        Self::linux_location()
    }

    fn linux_location() -> Result<PathBuf> {
        // TODO: WSL?
        for dir in LINUX_CHROME_DIRS.into_iter().map(PathBuf::from) {
            for file in LINUX_CHROME_FILES {
                let path = dir.join(file);
                if path.exists() {
                    return Ok(path);
                }
            }
        }
        Err(eyre!("Unable to find chrome executable"))
    }
}

#[test]
fn version_from_output_test() {
    assert_eq!(
        Version::version_from_output("Chromium 87.0.4280.141 snap").unwrap(),
        Version {
            major: 87,
            minor: 0,
            build: 4280,
            patch: 141
        }
    );
    assert_eq!(
        Version::version_from_output("127.0.0.1").unwrap(),
        Version {
            major: 127,
            minor: 0,
            build: 0,
            patch: 1
        }
    );
}

#[test]
#[should_panic]
fn version_from_output_panic_test() {
    assert_eq!(
        Version::version_from_output("a.0.0.1").unwrap(),
        Version {
            major: 127,
            minor: 0,
            build: 0,
            patch: 1
        }
    );
}

/// This module manages version selection of the `chromedriver`,
/// based on the installed browser version.
///
/// See https://chromedriver.chromium.org/downloads/version-selection
use eyre::{eyre, Result};
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
        let latest_release_url = format!(
            "{}/LATEST_RELEASE_{}",
            Self::BASE_URL,
            Version::find()?.build_version()
        );
        debug!("latest_release_url: {}", latest_release_url);
        let resp = reqwest::blocking::get(&latest_release_url)?;
        Ok(resp.text()?)
    }

    /// Returns the download url for the driver executable
    fn direct_download_url(&self, version: &str) -> Result<Url> {
        Ok(Url::parse(&format!(
            "{}/{version}/chromedriver_{platform}",
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

#[cfg(target_os = "linux")]
static LINUX_CHROME_DIRS: &[&'static str] = &[
    "/usr/local/sbin",
    "/usr/local/bin",
    "/usr/sbin",
    "/usr/bin",
    "/sbin",
    "/bin",
    "/opt/google/chrome",
];
#[cfg(target_os = "linux")]
static LINUX_CHROME_FILES: &[&'static str] =
    &["google-chrome", "chrome", "chromium", "chromium-browser"];

#[cfg(target_os = "windows")]
static WIN_CHROME_DIRS: &[&'static str] = &["\\Google\\Chrome\\Application", "\\Chromium\\Application"];
#[cfg(target_os = "windows")]
static WIN_CHROME_ROOTS: &[&'static str] = &["LOCALAPPDATA", "PROGRAMFILES"];

#[cfg(target_os = "macos")]
static MAC_CHROME_DIRS: &[&'static str] = &[
    "/Applications/Chromium.app",
    "/Applications/Google Chrome.app",
];
#[cfg(target_os = "macos")]
static MAC_CHROME_FILES: &[&'static str] =
    &["Contents/MacOS/Chromium", "Contents/MacOS/Google Chrome"];

impl Version {
    /// Returns the version of the currently installed Chrome/Chromium browser
    pub fn find() -> Result<Self> {
        #[cfg(target_os = "linux")]
        return Self::linux_version();
        #[cfg(target_os = "windows")]
        return Self::windows_version();
        #[cfg(target_os = "macos")]
        unimplemented!();
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

    #[cfg(target_os = "linux")]
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

    #[cfg(target_os = "windows")]
    fn windows_version() -> Result<Self> {
        let output = Command::new("pwsh.exe")
            .arg("--command")
            .arg(format!("(Get-ItemProperty '{}').VersionInfo.ProductVersion", Location::location()?.display()))
            .stdout(Stdio::piped())
            .output()?
            .stdout;

        let output = String::from_utf8(output)?;
        debug!("chrome version: {}", output);

        Ok(Self::version_from_output(&output)?)
    }

    #[cfg(target_os = "macos")]
    fn mac_version() -> Result<Self> {
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
            .ok_or(eyre!(
                "regex: Could not find 4-part Chrome version string in '{}'",
                output
            ))?
            .get(0)
            .map_or("", |m| m.as_str());
        let parts: Vec<i16> = version
            .split(".")
            .map(|i| i.parse::<i16>().unwrap())
            .collect();

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
        return Self::linux_location();
        #[cfg(target_os = "windows")]
        return Self::windows_location();
        #[cfg(target_os = "macos")]
        return Self::mac_location();
    }

    #[cfg(target_os = "linux")]
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

    #[cfg(target_os = "windows")]
    fn windows_location() -> Result<PathBuf> {
        for dir in WIN_CHROME_DIRS.into_iter().map(PathBuf::from) {
            for root in WIN_CHROME_ROOTS.into_iter().map(PathBuf::from) {
                let path = root.join(&dir).join("chrome.exe");
                if path.exists() {
                    return Ok(path);
                }
            }
        }
        Err(eyre!("Unable to find chrome executable"))
    }

    #[cfg(target_os = "macos")]
    fn mac_location() -> Result<PathBuf> {
        for dir in MAC_CHROME_DIRS.into_iter().map(PathBuf::from) {
            for file in MAC_CHROME_FILES {
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
#[should_panic(expected = "Could not find 4-part Chrome version string in 'a.0.0.1'")]
fn version_from_output_panic_test() {
    Version::version_from_output("a.0.0.1").unwrap();
}

#[test]
#[should_panic(expected = "Could not find 4-part Chrome version string in 'abc 1.0.1 def'")]
fn version_from_output_panic_not_4_parts_test() {
    Version::version_from_output("abc 1.0.1 def").unwrap();
}

#[test]
fn direct_download_url_test() {
    #[cfg(target_os = "linux")]
    assert_eq!(
        "https://chromedriver.storage.googleapis.com/v1/chromedriver_linux64.zip",
        Chromedriver::new()
            .direct_download_url("v1")
            .unwrap()
            .to_string()
    );
    #[cfg(target_os = "macos")]
    assert_eq!(
        "https://chromedriver.storage.googleapis.com/v1/chromedriver_mac64.zip",
        Chromedriver::new()
            .direct_download_url("v1")
            .unwrap()
            .to_string()
    );
    #[cfg(target_os = "windows")]
    assert_eq!(
        "https://chromedriver.storage.googleapis.com/v1/chromedriver_win32.zip",
        Chromedriver::new()
            .direct_download_url("v1")
            .unwrap()
            .to_string()
    );
}

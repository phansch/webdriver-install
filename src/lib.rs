pub mod chromedriver;
pub mod geckodriver;
pub mod installer;

// Re-export these so that users don't have to type something like
//
//     webdriver_installer::installer::install(...)
pub use installer::{install, install_into};
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

#[must_use]
#[cfg(target_os = "windows")]
fn run_powershell_cmd(cmd: &str) -> std::process::Output {
    use std::process::{Command, Stdio};
    use tracing::debug;

    let mut ps = Command::new("powershell");
    ps.stdin(Stdio::piped());
    ps.stdout(Stdio::piped());
    ps.stderr(Stdio::piped());

    let process = ps.args(&["-Command", cmd]).spawn().unwrap();

    let output = process.wait_with_output().expect("failed to wait on child process");
    debug!("stdout: {:?}", String::from_utf8(output.clone().stdout).unwrap());
    debug!("stderr: {:?}", String::from_utf8(output.clone().stderr).unwrap());
    output
}

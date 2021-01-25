//! Fast and simple webdriver installation
//!
//! ## Usage
//!
//! By default, driver executables are installed into `$HOME/.webdrivers`.
//!
//! ```no_run
//! # fn main() -> eyre::Result<()> {
//! use webdriver_install::Driver;
//!
//! // Install geckodriver into $HOME/.webdrivers
//! Driver::Gecko.install()?;
//!
//! // Install chromedriver into $HOME/.webdrivers
//! Driver::Chrome.install()?;
//! # Ok(())
//! # }
//! ```
//!
//! You can specify a different location with [`Driver::install_into`]:
//!
//! ```no_run
//! # fn main() -> eyre::Result<()> {
//! use webdriver_install::Driver;
//! use std::path::PathBuf;
//!
//! // Install geckodriver into /tmp/webdrivers
//! Driver::Gecko.install_into(PathBuf::from("/tmp/webdrivers"))?;
//!
//! // Install chromedriver into /tmp/webdrivers
//! Driver::Chrome.install_into(PathBuf::from("/tmp/webdrivers"))?;
//! # Ok(())
//! # }
//! ```

mod chromedriver;
mod geckodriver;
pub mod installer;

use eyre::Result;
pub use installer::Driver;
use url::Url;

#[doc(hidden)]
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

    let output = process
        .wait_with_output()
        .expect("failed to wait on child process");
    debug!(
        "stdout: {:?}",
        String::from_utf8(output.clone().stdout).unwrap()
    );
    debug!(
        "stderr: {:?}",
        String::from_utf8(output.clone().stderr).unwrap()
    );
    output
}

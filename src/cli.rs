use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use eyre::{eyre, Result};
use std::path::PathBuf;
use webdriver_install::Driver;

pub fn run() -> Result<PathBuf> {
    let supported_drivers: &[&str] = &vec![Driver::Chrome, Driver::Gecko]
        .iter()
        .map(Driver::as_str)
        .collect::<Vec<&str>>();
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("install")
                .short("i")
                .long("install")
                .value_name("DRIVER")
                .case_insensitive(true)
                .possible_values(supported_drivers)
                .takes_value(true)
                .help("Install the specified DRIVER"),
        )
        .arg(
            Arg::with_name("dir")
                .long("dir")
                .value_name("DIR")
                .requires("install")
                .takes_value(true)
                .help("Override the default installation directory"),
        )
        .get_matches();

    if let Some(val) = matches.value_of("install") {
        if let Some(dir) = matches.value_of("dir") {
            return match Driver::from_str(val).unwrap() {
                Driver::Chrome => Driver::Chrome.install_into(PathBuf::from(dir)),
                Driver::Gecko => Driver::Gecko.install_into(PathBuf::from(dir)),
            };
        } else {
            return match Driver::from_str(val).unwrap() {
                Driver::Chrome => Driver::Chrome.install(),
                Driver::Gecko => Driver::Gecko.install(),
            };
        }
    }
    Err(eyre!("what do ya wanna do?"))
}

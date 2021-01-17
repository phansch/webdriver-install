use std::process::Command;

use tempfile::tempdir;
use webdriver_install::Driver;

#[cfg(target_os = "linux")]
#[test]
fn chromedriver_install_test() {
    let target_dir = tempdir().unwrap();
    let executable_path = target_dir.path().join("chromedriver");
    webdriver_install::install_into(Driver::Chrome, target_dir.into_path()).unwrap();

    let output = Command::new(executable_path)
        .arg("--version")
        .output()
        .unwrap();
    assert!(output.status.success());
}

#[cfg(target_os = "linux")]
#[test]
fn geckodriver_install_test() {
    let target_dir = tempdir().unwrap();
    let executable_path = target_dir.path().join("geckodriver");
    webdriver_install::install_into(Driver::Gecko, target_dir.into_path()).unwrap();

    let output = Command::new(executable_path)
        .arg("--version")
        .output()
        .unwrap();
    assert!(output.status.success());
}

use std::process::{Command, Stdio};
use std::io::Write;

use tempfile::tempdir;
use webdriver_install::Driver;

#[cfg(any(target_os = "linux", target_os = "macos"))]
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

#[cfg(any(target_os = "linux", target_os = "macos"))]
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

#[cfg(target_os = "windows")]
fn run_powershell_cmd(cmd: &str) {
    let mut ps = Command::new("powershell");
    ps.stdin(Stdio::piped());
    ps.stdout(Stdio::piped());
    ps.stderr(Stdio::piped());

    let mut process = ps.args(&["-Command", "-"]).spawn().unwrap();
    let stdin = process.stdin.as_mut().expect("could not get stdin");
    stdin.write_all(cmd.as_bytes()).unwrap();

    let output = process.wait_with_output().expect("failed to wait on child process");
    println!("stdout: {:?}", String::from_utf8(output.stdout).unwrap());
    println!("stderr: {:?}", String::from_utf8(output.stderr).unwrap());
}

#[cfg(target_os = "windows")]
#[test]
fn chromedriver_install_test_win() {
    let target_dir = tempdir().unwrap();
    let target_dir = target_dir.path();
    let executable_path = target_dir.join("chromedriver.exe");
    run_powershell_cmd(&format!("Get-ChildItem {:?}", target_dir));

    webdriver_install::install_into(Driver::Chrome, target_dir.to_path_buf()).unwrap();


    // assert!(output.status.success());
}

#[cfg(target_os = "windows")]
#[test]
fn geckodriver_install_test_win() {
    let target_dir = tempdir().unwrap();
    let executable_path = target_dir.path().join("geckodriver.exe");
    webdriver_install::install_into(Driver::Gecko, target_dir.into_path()).unwrap();

    let output = Command::new(executable_path)
        .arg("--version")
        .output()
        .unwrap();
    assert!(output.status.success());
}

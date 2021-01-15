use webdriver_install::Driver;

fn main() {
    webdriver_install::install_latest(Driver::Gecko).unwrap();
}

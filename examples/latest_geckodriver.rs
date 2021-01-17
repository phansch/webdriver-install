use webdriver_install::Driver;

fn main() {
    webdriver_install::install(Driver::Gecko).unwrap();
}

use eyre::Result;
use webdriver_install::Driver;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    Driver::Gecko.install()?;

    Ok(())
}

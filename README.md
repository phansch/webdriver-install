# webdriver-install

[![Rust](https://github.com/phansch/webdriver-install/workflows/Rust/badge.svg)](https://github.com/phansch/webdriver-install/actions)
[![Sponsor
count](https://img.shields.io/badge/sponsors-4-brightgreen)](https://phansch.net/thanks)
[![Crates.io](https://img.shields.io/crates/v/webdriver-install)](https://crates.io/crates/webdriver-install)
[![docs.rs](https://docs.rs/webdriver-install/badge.svg)](https://docs.rs/webdriver-install/)
![MSRV](https://img.shields.io/badge/MSRV-1.44+-lightgray.svg)

Fast and simple webdriver installation

## Usage

```rust
use webdriver_install::Driver;
use std::path::PathBuf;

fn main() {
    // Install into $HOME/.webdrivers
    Driver::Chrome.install();

    // Install into specified path
    Driver::Chrome.install_into(PathBuf::from("/tmp/webdrivers"));
}
```

## Implementation state

Driver installation support:

 * ✅ `chromedriver`
 * ✅ `geckodriver`
 * ⬜ `edgedriver`
 * ⬜ `iedriver`
 * ⬜ `operadriver`
 * ⬜ `safaridriver`
      Safaridriver comes pre-installed on all MacOS systems, but we can at least
      provide the binary location.

Usability:

 * ✅ README instructions
 * ✅ Stable-ish library API
 * ⬜ Command line interface
 * ⬜ Updating of installed drivers
 * ⬜ Removal of installed drivers
 * ✅ Published on crates.io
 * ✅ Published library docs
 * ✅ Linux support
 * ✅ Windows support
 * ✅ MacOS support
 * ⬜ WSL support
 * ⬜ Pre-built binaries

Inspired by:

 * [titusfortner/webdrivers](https://github.com/titusfortner/webdrivers/) which is written in Ruby.
 * `wasm-pack`'s [internal test helpers](https://github.com/rustwasm/wasm-pack/tree/master/src/test/webdriver?rgh-link-date=2021-01-14T06%3A59%3A33Z)
 * [bonigarcia/webdrivermanager](https://github.com/bonigarcia/webdrivermanager)

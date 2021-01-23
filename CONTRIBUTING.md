# Contributing to webdriver-install

All contributors are expected to follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).

## Releasing a new version

Make sure cargo-release is installed:

    cargo install cargo-release

Do a dry-run before doing the actual release:

    cargo release <level> --dry-run

Create the actual release:

    cargo release <level>

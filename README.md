# ddo-build-planner

A Cross-Platform Dungeons & Dragons Online Build Planner written in Rust.

The goal of this project is to make a ddo build planner that's blazingly fast, and runs on any system. Relatively high potential of a corredsponding website-based version in the future.

## Installing

While this is in very early stages of development, you are more than welcome to install from source using the following command (after you've set up cargo)

```sh
cargo install --git https://www.github.com/LittleTealeaf/ddo-build-planner app --force
```


## Building from source

To build from source, you will need to install **Rust** and **Cargo** (and of course **Git**). You can find more information at [rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

After cloning this (or a forked version of this) repository, navigate to the directory and you can run any of the following commands.

```sh
cargo build # Builds the app
cargo build --release # Builds the release-ready version
cargo run # Builds and runs a dev build
cargo test --all # Builds and runs tests
cargo doc --open --all --no-deps # Builds the documentation and opens it up in a browser.
```

If you want to dive through the documentation, `cargo doc --open` will land you in the `app` crate. You can either navigate to the `builder` crate, or run the following command to just document the core library (where all of the good builder stuff happens)

```sh
cargo doc -p builder --open
```

## Contributing

Steps on how to contribute can be found in [CONTRIBUTING.md](./CONTRIBUTING.md).

## Inspiration

The idea of this builder is inspired by the amazing work of Maetrim on their [DDO Builder](https://github.com/Maetrim/DDOBuilder).

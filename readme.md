# Rust template for Iskra JS or Iskra JS Mini boards

## Prerequisites

### Install Rustup and Rust

If you don't have `rustup` installed yet, follow the instructions on the [rustup.rs](https://rustup.rs) site

### Install Rust plugins
```sh
rustup default stable
rustup target add thumbv7em-none-eabihf
rustup component add llvm-tools-preview
```

### Install VS Code and plugins

If you don't have `VS Code` installed yet, follow the instructions on the [code.visualstudio.com](https://code.visualstudio.com) site

Install plugins:
```sh
code --install-extension matklad.rust-analyzer
code --install-extension bungcip.better-toml
code --install-extension marus25.cortex-debug
```

### Install Cargo Sub-Commands and tools

```sh
cargo install cargo-generate cargo-binutils probe-run cargo-embed uf2conv
```

## Generate the project

```sh
cargo generate gh:amperka/iskrajs-rust-template
```

## Build

In the root of the generated project.

```sh
cargo build --release
```

## Flash

In the root of the generated project.

To build and flash using uf2 bootloader, use:
```sh
# for Windows or MacOS
cargo objcopy --release -- -O binary ./target/iskrajs.bin
uf2conv ./target/iskrajs.bin --base 0x08008000 --family 0x57755a57 --output iskrajs.uf2
# copy iskrajs.uf2 file to IskraJSBOOT or IskraJSMini disk

# for Linux
sh ./load.sh
```

To build and flash using debugger, use:
```sh
cargo embed --release
```

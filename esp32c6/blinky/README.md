# ESP32C6 Multicore Blinky Example

This is a companion example to an as yet unreleased blog post on using multiple core microcontrollers.

## Usage

This example was tested on [ESP32-C6-DevKitC-1 v1.1](https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32c6/esp32-c6-devkitc-1/user_guide_v1.1.html)

To run the example you'll need to have a proper rust environment setup. You can follow [The Rust on ESP book](https://docs.esp-rs.org/book/installation/riscv.html) RISC-V target install guide. And I used [`probe-rs`](https://docs.esp-rs.org/book/tooling/debugging/probe-rs.html) for debugging

First compile the `blinky-lp` crate

```sh
cd blink-lp
cargo build --release
```

then flash the `blinky-hp` crate

```sh
cd blinky-hp
cargo run --release
```

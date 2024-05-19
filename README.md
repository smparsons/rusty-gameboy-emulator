# WebBoy

This repo holds the core logic for WebBoy: a Game Boy emulator written in Rust. The code can be compiled down to WebAssembly so it can be played on the web.

This emulator passes all [CPU instruction tests](https://github.com/retrio/gb-test-roms/tree/master/cpu_instrs) from Blargg's test ROM collection.

## How to Use

1. Clone [webboy-client](https://github.com/smparsons/webboy-client) to your local machine. The project should live under the same directory as webboy-core.
2. To build the project, simply run `cargo build`.
3. To compile the implementation to WebAssembly, you will first need to install wasm-pack with the command `cargo install wasm-pack`. Then, run `sh ./build.sh` to generate the Javascript binding code in the webboy-client project.

## Test Suite

This project holds a fairly extensive test suite, as the bulk of the logic was designed using a TDD approach. There are a lot of tests that exercise CPU opcodes, and basic tests that exercise the GPU. Run `cargo test` to run the test suite.

## Supported Features

This emulator is still a work in progress and not all features are supported.

| Feature           | Supported |
| ----------------- | --------- |
| CPU               | ✅        |
| Basic Graphics    | ✅        |
| Audio             | ❌        |
| Color Support     | ❌        |
| GameShark Support | ❌        |

### MBC Support

At the moment, only MBC1 is supported.

| Type | Supported |
| ---- | --------- |
| MBC1 | ✅        |
| MBC2 | ❌        |
| MBC3 | ❌        |
| MBC4 | ❌        |
| MBC5 | ❌        |
| MBC6 | ❌        |
| MBC7 | ❌        |

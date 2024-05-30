# Oxidized Turret

The tower defense game you were always searching for.

## How to build

Install Rust: https://www.rust-lang.org/tools/install

Then run on the commandline:
```bash
cargo build
cargo run
```

## How to debug

Install VSCode + Extensions:
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [Native Debug](https://marketplace.visualstudio.com/items?itemName=webfreak.debug)

In main.rs file: click debug at line 0. You can also use breakpoints.

## WebAssembly - wasm

Great documentation available [here](https://rustwasm.github.io/docs/book/game-of-life/introduction.html).

Summary:
- Setup: Install wasm-pack, npm
```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
npm install npm@latest -g
```
- Build wasm binary and js bindings:
```bash
wasm-pack build --out-dir web/src/wasm
```

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

## ToDo

### Rust engine

- [x] remove creeps when they die
- [x] path finding for creeps (towers block creeps)
- [x] tower targeting logic (with range)
- [x] you loose health if creep reaches target
- [x] you win if all creeps of a level are dead
- [ ] game phases (build, animation)
- [x] build towers
- [ ] multiplayer (hard!)
- [ ] tech tree
- [ ] creeps have multiple intermediate goals
- [ ] creep types, e.g. speed, health, flying, hero (not effected by spells - good idea: spells), splitting (e.g. https://infinitode-2.fandom.com/wiki/Enemies)
- [ ] ingame currency
- [ ] turret types, e.g. splash, sniper, freeze, aoe, poison, anti-air (e.g. https://infinitode-2.fandom.com/wiki/Towers)

### js frontend

- [x] Zoom and Drag
- [x] Engine Coordinates to Pixel coordinates
- [x] fullscreen + resize
- [x] Simple Tower Building UI
- [x] display path of creeps
- [ ] display score, health, current level, and game resolution (who won)
- [ ] display tower range on click
- [ ] higher frame rates than 60 FPS
- [ ] support mobile devices (drag, pinch zoom, ...)
- [ ] remove towers

### Other

- [ ] configure github pages deployment

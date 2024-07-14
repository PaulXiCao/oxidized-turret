# Oxidized Turret Web Frontend

You need to install the Node.js >= 22.
Also, you need at least wasm-pack 0.13.0. (update with `cargo install wasm-pack`)

## Dev Server (Singleplayer only)

Run `node server.mjs` in this folder.
Afterwards, the game should run on http://localhost:8080/index.html.

## Multiplayer Server

Run

```bash
BASIC_AUTH_USER=admin BASIC_AUTH_PASSWORD=secret node prod-server.mjs
```

in this folder.

Afterwards, the multiplayer game server should run on http://localhost:1337

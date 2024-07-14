import http from "node:http";
import fs from "node:fs";
import crypto from "node:crypto";
import { BaseLayout } from "./server/BaseLayout.js";
import path from "node:path";
import { SelectScreen, SelectScreenStyles } from "./server/SelectScreen.js";
import { Lobbies } from "./server/Lobbies.js";
import { CreateLobby, CreateLobbyStyles } from "./server/CreateLobby.js";
import { Readable } from "node:stream";
import { LobbyDetail } from "./server/LobbyDetail.js";
import { createNewGame } from "./server/gameFactory.js";
import { Multiplayer, MultiplayerStyles } from "./server/Multiplayer.js";
import * as wasm from "./src/wasm/oxidized_turret_bg.js";
import { Singleplayer, SingleplayerStyles } from "./server/Singleplayer.js";

const MIME_TYPES = {
  html: "text/html; charset=utf-8",
  js: "application/javascript",
  wasm: "application/wasm",
  json: "application/manifest+json",
  png: "image/png",
};

/**
 *
 * @param {string} authHeader
 * @returns {{username: string, password: string} | null}
 */
function getBasicAuthCredentials(authHeader) {
  if (!authHeader.startsWith("Basic ")) {
    return null;
  }
  const decoded = Buffer.from(
    authHeader.slice("Basic ".length),
    "base64"
  ).toString("utf-8");

  const parts = decoded.split(":");
  if (parts.length !== 2) {
    return null;
  }

  return { username: parts[0], password: parts[1] };
}

/**
 *
 * @param {string} cookieHeader
 * @returns {Record<string, string>}
 */
function parseCookies(cookieHeader) {
  const parts = cookieHeader.split(";");
  const cookies = {};
  for (const part of parts) {
    const keyValue = part.split("=");
    if (keyValue.length !== 2) {
      continue;
    }
    const [key, value] = keyValue;
    cookies[decodeURIComponent(key.trim())] = decodeURIComponent(value.trim());
  }
  return cookies;
}

function isFile(filePath) {
  return new Promise((resolve) => {
    fs.stat(filePath, (err, stats) => {
      if (err) {
        return resolve(false);
      }
      return resolve(stats.isFile());
    });
  });
}

function normalizePath(basePath, untrustedPath) {
  const normalized = path.join("/", untrustedPath);
  return path.join(basePath, normalized);
}

function send404(res) {
  res.statusCode = 404;
  res.setHeader("content-type", "text/html; charset=utf-8");
  res.write(BaseLayout({ title: "Error", body: "404 Not Found", styles: "" }));
  res.end();
}

function sendBasicAuthRequired(res) {
  res.statusCode = 401;
  res.setHeader("WWW-Authenticate", "Basic");
  res.end();
}

function createRandomId() {
  return crypto.randomBytes(20).toString("base64url");
}

const SESSIONS = {};
const LOBBIES = {};
const CONNECTIONS = {};
const GAMES = {};
let connectionId = 0;

const server = http.createServer(async (req, res) => {
  const url = new URL(req.url, `http://${req.headers.host}`);
  const path = url.pathname;

  // serve public static ressources from src without credential check
  const normalizedPath = normalizePath("src", path);
  if (await isFile(normalizedPath)) {
    const extension = normalizedPath.split(".").at(-1);
    if (!MIME_TYPES[extension]) {
      return send404(res);
    } else {
      const stream = fs.createReadStream(normalizedPath);
      res.statusCode = 200;
      res.setHeader("content-type", MIME_TYPES[extension]);
      stream.pipe(res);
      return;
    }
  }

  const credentials = getBasicAuthCredentials(req.headers?.authorization || "");
  const cookies = parseCookies(req.headers?.cookie || "");

  if (
    !(
      credentials !== null &&
      credentials.username === process.env.BASIC_AUTH_USER &&
      credentials.password === process.env.BASIC_AUTH_PASSWORD
    ) &&
    !SESSIONS[cookies.SESSION]
  ) {
    return sendBasicAuthRequired(res);
  }

  const sessionKey = cookies.SESSION || createRandomId();
  if (!SESSIONS[sessionKey]) {
    SESSIONS[sessionKey] = { connections: {} };
  }

  // refresh session on each authorized request
  res.setHeader(
    "Set-Cookie",
    `SESSION=${sessionKey}; HttpOnly; SameSite=Strict; Max-Age=34560000`
  );

  if (path === "/") {
    res.statusCode = 200;
    res.setHeader("content-type", "text/html; charset=utf-8");
    res.write(
      BaseLayout({
        title: "Oxidized Turret",
        body: SelectScreen(),
        styles: SelectScreenStyles,
      })
    );
    res.end();
    return;
  }

  if (path === "/lobbies") {
    const lobbies = Object.values(LOBBIES).filter(({ isPublic }) => isPublic);

    res.statusCode = 200;
    res.setHeader("content-type", "text/html; charset=utf-8");
    res.write(
      BaseLayout({
        title: "Lobbies - Oxidized Turret",
        body: Lobbies({ lobbies }),
        styles: "",
      })
    );
    res.end();
    return;
  }

  if (path === "/lobbies/create" && req.method === "GET") {
    res.statusCode = 200;
    res.setHeader("content-type", "text/html; charset=utf-8");
    res.write(
      BaseLayout({
        title: "Create Lobby - Oxidized Turret",
        body: CreateLobby(),
        styles: CreateLobbyStyles,
      })
    );
    res.end();
    return;
  }

  if (path === "/lobbies/create" && req.method === "POST") {
    // create lobby
    const request = new Request(url, {
      method: req.method,
      headers: req.headers,
      body: Readable.toWeb(req),
      duplex: "half",
    });
    const formData = await request.formData();

    const lobby = {
      id: createRandomId(),
      name: formData.get("name"),
      numPlayers: Number(formData.get("num-players")),
      isPublic: formData.get("public") === "on",
      players: {},
    };
    LOBBIES[lobby.id] = lobby;
    console.log(lobby);

    res.statusCode = 303;
    res.setHeader("Location", `/lobbies/detail/${lobby.id}`);
    res.end();
    return;
  }

  if (path.startsWith("/lobbies/detail/")) {
    const lobbyId = path.slice("/lobbies/detail/".length);
    const lobby = LOBBIES[lobbyId];
    if (!lobby) {
      return send404(res);
    }

    lobby.players[sessionKey] = { id: sessionKey };

    res.statusCode = 200;
    res.setHeader("content-type", "text/html; charset=utf-8");
    res.write(
      BaseLayout({
        title: "Lobby - Oxidized Turret",
        body: LobbyDetail(lobby),
        styles: "",
      })
    );
    res.end();
    return;
  }

  if (path.startsWith("/play/")) {
    if (path.endsWith("/sse")) {
      res.writeHead(200, {
        "content-type": "text/event-stream",
        "cache-control": "no-cache, no-store, must-revalidate, max-age=0",
        connection: "keep-alive",
      });

      let currentConnectionId = connectionId++;
      CONNECTIONS[currentConnectionId] = res;
      SESSIONS[sessionKey].connections[currentConnectionId] = true;

      req.on("close", () => {
        delete CONNECTIONS[currentConnectionId];
        delete SESSIONS[sessionKey].connections[currentConnectionId];
        console.log(`Removed connection with id ${currentConnectionId}.`);
      });
      return;
    } else if (req.method === "GET") {
      const lobbyId = path.slice("/play/".length);
      const lobby = LOBBIES[lobbyId];
      if (!lobby) {
        return send404(res);
      }

      lobby.players[sessionKey] = { id: sessionKey };

      if (!GAMES[lobbyId]) {
        GAMES[lobbyId] = createNewGame();
      }

      res.statusCode = 200;
      res.setHeader("content-type", "text/html; charset=utf-8");
      res.write(
        BaseLayout({
          title: "Play - Oxidized Turret",
          body: Multiplayer(),
          styles: MultiplayerStyles,
        })
      );
      res.end();
      return;
    } else if (req.method === "POST") {
      const lobbyId = path.slice("/play/".length);
      const lobby = LOBBIES[lobbyId];
      if (!lobby) {
        return send404(res);
      }

      /** @type{wasm.Game} */
      const game = GAMES[lobbyId];
      if (!game) {
        return send404(res);
      }

      const request = new Request(url, {
        method: req.method,
        headers: req.headers,
        body: Readable.toWeb(req),
        duplex: "half",
      });
      const message = await request.json();
      if (message.type === "build_tower") {
        game.build_tower(message.data.x, message.data.y, message.data.kind);
      } else if (message.type === "start_wave") {
        game.start_wave();
      } else if (message.type === "upgrade_tower") {
        game.upgrade_tower(message.data.id, message.data.index);
      } else if (message.type === "sell_tower") {
        game.sell_tower(message.data.id, message.data.index);
      }

      for (const player of Object.values(lobby.players)) {
        const session = SESSIONS[player.id];
        for (const connectionId of Object.keys(session.connections)) {
          const connection = CONNECTIONS[connectionId];
          if (!connection) {
            console.warn(`Connection not found!`);
          }
          connection.write(`data: ${JSON.stringify(message)}\n\n`);
        }
      }

      res.statusCode = 200;
      res.end();
      return;
    }
  }

  if (path === "/play") {
    res.statusCode = 200;
    res.setHeader("content-type", "text/html; charset=utf-8");
    res.write(
      BaseLayout({
        title: "Play - Oxidized Turret",
        body: Singleplayer(),
        styles: SingleplayerStyles,
      })
    );
    res.end();
    return;
  }

  return send404(res);
});

const port = 1337;
server.listen(port);
console.log("Listening at: http://localhost:" + port);

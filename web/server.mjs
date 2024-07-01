import http from "node:http";
import fs from "node:fs";
import child_process from "node:child_process";

const MIME_TYPES = {
  html: "text/html; charset=utf-8",
  js: "application/javascript",
  wasm: "application/wasm",
  json: "application/manifest+json",
  png: "image/png",
};

let connectionId = 0;
const connections = {};

const server = http.createServer((req, res) => {
  const url = new URL(req.url, `http://${req.headers.host}`);
  const path = url.pathname;

  if (path === "/sse") {
    res.writeHead(200, {
      "content-type": "text/event-stream",
      "cache-control": "no-cache, no-store, must-revalidate, max-age=0",
      connection: "keep-alive",
    });

    let currentConnectionId = connectionId++;
    connections[currentConnectionId] = res;
    req.on("close", () => {
      delete connections[currentConnectionId];
      console.log(`Removed connection with id ${currentConnectionId}.`);
    });
    return;
  }

  const extension = path.split(".").at(-1);
  if (!MIME_TYPES[extension]) {
    res.statusCode = 404;
    res.setHeader("content-type", "text/plain");
    res.end("404");
    return;
  }

  const stream = fs.createReadStream("./src" + path);
  stream.on("open", () => {
    res.statusCode = 200;
    res.setHeader("content-type", MIME_TYPES[extension]);
  });
  stream.on("error", () => {
    res.statusCode = 404;
    res.setHeader("content-type", "text/plain");
    res.end("404");
  });

  stream.pipe(res);
});

let currentBuild = null;
fs.watch(
  "../src",
  { persistent: true, recursive: true },
  function rustListener(eventType, filename) {
    if (currentBuild) {
      return;
    }
    console.log(eventType, filename, "rebuilding WASM");
    currentBuild = child_process.spawn("wasm-pack", [
      "build",
      "--out-dir",
      "web/src/wasm",
    ]);

    currentBuild.on("exit", () => {
      currentBuild.removeAllListeners();
      currentBuild = null;
    });
  }
);

function reloadClients() {
  for (const res of Object.values(connections)) {
    console.log("event: reload");
    res.write("event: reload\ndata: \n\n");
  }
}

fs.watch(
  "./src",
  { persistent: true, recursive: true },
  function jsListener(eventType, filename) {
    debounce(reloadClients, 500);
  }
);

const debounceMap = new WeakMap();
function debounce(cb, ms) {
  if (debounceMap.has(cb)) {
    clearTimeout(debounceMap.get(cb));
  }
  const id = setTimeout(cb, ms);
  debounceMap.set(cb, id);
}

const port = 8080;
server.listen(port);
console.log("Listening at: http://localhost:" + port + "/index.html ...");

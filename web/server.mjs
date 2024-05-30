import http from "node:http";
import fs from "node:fs";

const MIME_TYPES = {
  html: "text/html; charset=utf-8",
  js: "application/javascript",
  wasm: "application/wasm",
};

const server = http.createServer((req, res) => {
  const url = new URL(req.url, `http://${req.headers.host}`);
  const path = url.pathname;

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

const port = 8080;
server.listen(port);
console.log("Listening at: http://localhost:" + port + "/index.html ...");

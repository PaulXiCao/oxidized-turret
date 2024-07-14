import { initGame } from "./main.js";

let receiveMessage = undefined;
async function sendMessage(message) {
  receiveMessage(message);
}

const wasmPath = "./wasm/oxidized_turret_bg.wasm";

if (document.location.port !== "8080") {
  const { receiveMessage: _receiveMessage } = await initGame({
    sendMessage,
    wasmPath,
  });
  receiveMessage = _receiveMessage;
} else {
  /**
   * Setup Hot-Module-Reloading
   *
   * 0) If an existing state exists load the game from this state and delete the state afterwards.
   * 1) Listen to the Server-Sent-Event source
   * 2) On changes save state to the indexedDB and reload the website
   */
  const request = window.indexedDB.open("StateStore", 1);
  let db = null;
  let getState = null;
  request.onsuccess = (event) => {
    db = event.target.result;

    const transaction = db.transaction(["state"], "readwrite");
    transaction.oncomplete = (event) => {};

    const objectStore = transaction.objectStore("state");

    const getRequest = objectStore.get("state");
    getRequest.onsuccess = async (event) => {
      const { getState: _getState, receiveMessage: _receiveMessage } =
        await initGame({
          state: getRequest.result,
          sendMessage,
          wasmPath,
        });
      receiveMessage = _receiveMessage;
      getState = _getState;
    };
    getRequest.onerror = (event) => {
      console.log("error while retrieving", event);
    };

    const deleteRequest = objectStore.delete("state");
    deleteRequest.onsuccess = (event) => {
      connectEventSource();
    };
  };
  request.onupgradeneeded = (event) => {
    const db = event.target.result;
    const objectStore = db.createObjectStore("state");
  };
  request.onerror = console.error;

  function connectEventSource() {
    const source = new EventSource("/sse");

    let isReloading = false;
    source.addEventListener("reload", function eventSourceListener(event) {
      if (isReloading) return;
      isReloading = true;
      const transaction = db.transaction(["state"], "readwrite");
      const objectStore = transaction.objectStore("state");
      const storeRequest = objectStore.put(getState(), "state");
      storeRequest.onsuccess = (event) => {
        console.log("State saved succesfully!");
      };
      storeRequest.onerror = console.error;

      transaction.oncomplete = (event) => {
        document.location.reload();
      };
    });

    source.addEventListener("error", async () => {
      console.log("SSE connection closed. Trying to reconnect...");
      await new Promise((res) => setTimeout(res, 500));
      connectEventSource();
    });
  }
}

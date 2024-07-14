import { html } from "./html.js";
import { Game, GameStyles } from "./Game.js";

export const Multiplayer = () => html`
  ${Game()}

  <script type="module">
    import { initGame } from "/js/main.js";

    async function sendMessage(message) {
      return fetch(window.location.href, {
        method: "POST",
        headers: {
          "content-type": "application/json",
        },
        body: JSON.stringify(message),
      });
    }

    const { receiveMessage } = await initGame({
      sendMessage,
      wasmPath: "/wasm/oxidized_turret_bg.wasm",
    });

    const serverEvents = new EventSource(window.location.href + "/sse");

    serverEvents.addEventListener("message", (event) => {
      const message = JSON.parse(event.data);
      receiveMessage(message);
    });
  </script>
`;

export const MultiplayerStyles = GameStyles;

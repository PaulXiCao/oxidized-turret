import { html } from "./html.js";
import { Game, GameStyles } from "./Game.js";

export const Singleplayer = () => html`
  ${Game()}

  <script type="module">
    import { initGame } from "/js/main.js";

    async function sendMessage(message) {
      return receiveMessage(message);
    }

    const { receiveMessage } = await initGame({
      sendMessage,
      wasmPath: "/wasm/oxidized_turret_bg.wasm",
    });
  </script>
`;

export const SingleplayerStyles = GameStyles;

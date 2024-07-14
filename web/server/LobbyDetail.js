import { html } from "./html.js";

export const LobbyDetail = ({ id, name, numPlayers, players }) => html`
  <div>Lobby-Id: ${id}</div>
  <div>Lobby-Name: ${name}</div>
  <div>Max. Players: ${numPlayers}</div>
  <div>Players in Lobby</div>
  <ul>
    ${Object.values(players)
      .map((player) => html`<li>${player.id}</li>`)
      .join("")}
  </ul>
  <a href="/play/${id}">Start Game</a>
`;

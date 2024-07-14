import { html } from "./html.js";

export const Lobbies = ({ lobbies }) => html`
  <a href="/lobbies/create">Open new Lobby</a>
  <div>Open Lobbies</div>
  <ul>
    ${lobbies
      .map(
        (lobby) =>
          html`<li><a href="/lobbies/detail/${lobby.id}">${lobby.name}</a></li>`
      )
      .join("")}
  </ul>
`;

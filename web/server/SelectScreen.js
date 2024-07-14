import { html } from "./html.js";

export const SelectScreen = () => html`
  <a href="/play" class="button">Local Game</a>
  <a href="/lobbies" class="button">Online Game</a>
`;

export const SelectScreenStyles = html`
  <style>
    .button {
      padding: 1px 6px;
      border: 1px outset buttonborder;
      border-radius: 3px;
      color: buttontext;
      background-color: buttonface;
      text-decoration: none;
    }
  </style>
`;

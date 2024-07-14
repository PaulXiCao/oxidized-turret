import { html } from "./html.js";

export const CreateLobby = () => html`
  <form action="/lobbies/create" method="POST">
    <p>Create a new Lobby</p>

    <div class="form-row">
      <label>Name</label>
      <input name="name" type="text" required />
    </div>
    <div class="form-row">
      <label>Number of Players</label>
      <input
        name="num-players"
        type="number"
        min="1"
        step="1"
        value="2"
        required
      />
    </div>
    <div class="form-row">
      <label>Public Lobby</label>
      <input name="public" type="checkbox" checked />
    </div>

    <button type="submit">Create</button>
  </form>
`;

export const CreateLobbyStyles = html`
  <style>
    label,
    input {
      display: block;
    }
    .form-row {
      margin-bottom: 10px;
    }
  </style>
`;

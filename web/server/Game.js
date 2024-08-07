import { html } from "./html.js";

export const Game = () => html`
  <canvas id="canvas"></canvas>
  <canvas id="ui-canvas"></canvas>
  <div id="ui">
    <div class="health">Health: 20</div>
    <div class="wave">Wave: 1</div>
    <div class="gold">Gold: 200</div>
    <div class="speed-controls">
      <div class="speed">Speed: 1</div>
      <div class="increase-speed">+</div>
      <div class="decrease-speed">-</div>
    </div>

    <div class="result">You've won at wave 10!</div>

    <div class="start">Start</div>

    <div class="building-sidebar">
      <div class="close">></div>
      <div class="towers">
        <div class="turret" data-type="Basic">
          Basic
          <div class="cost">50</div>
        </div>
        <div class="turret" data-type="Sniper">
          Sniper
          <div class="cost">80</div>
        </div>
        <div class="turret" data-type="Cannon">
          Cannon
          <div class="cost">60</div>
        </div>
        <div class="turret" data-type="Freezing">
          Freezing
          <div class="cost">80</div>
        </div>
        <div class="turret" data-type="Antiair">
          Antiair
          <div class="cost">40</div>
        </div>
        <div class="turret" data-type="Splash">
          Splash
          <div class="cost">80</div>
        </div>
        <div class="turret" data-type="Blast">
          Blast
          <div class="cost">75</div>
        </div>
        <div class="turret" data-type="Multishot">
          Multi- shot
          <div class="cost">90</div>
        </div>
        <div class="turret" data-type="Minigun">
          Minigun
          <div class="cost">110</div>
        </div>
        <div class="turret" data-type="Venom">
          Venom
          <div class="cost">100</div>
        </div>
        <div class="turret" data-type="Tesla">
          Tesla
          <div class="cost">100</div>
        </div>
        <div class="turret" data-type="Missile">
          Missile
          <div class="cost">150</div>
        </div>
        <div class="turret" data-type="Flamethrower">
          Flame- thrower
          <div class="cost">110</div>
        </div>
        <div class="turret" data-type="Laser">
          Laser
          <div class="cost">100</div>
        </div>
      </div>
      <div class="message">Click again to buy.</div>
    </div>

    <div class="tower-detail-sidebar">
      <div class="close">></div>
      <div class="tower-stats"></div>
      <div class="tower-detail-buttons">
        <div class="tower-sell">Sell</div>
        <div class="tower-upgrade">Upgrade</div>
      </div>
    </div>
  </div>
`;

export const GameStyles = html`
  <style>
    * {
      padding: 0;
      margin: 0;
      box-sizing: border-box;
      -webkit-user-select: none;
      user-select: none;
    }

    :root {
      font-family: "Roboto Condensed", sans-serif;
      font-optical-sizing: auto;
      font-weight: 400;
      font-style: normal;
    }

    body {
      height: 100vh;
      width: 100vw;
      overflow: hidden;
      touch-action: none;
    }

    #canvas {
      position: absolute;
      background: black;
      top: 0;
      left: 0;
      z-index: 0;
    }

    #ui-canvas {
      position: absolute;
      top: 0;
      left: 0;
      z-index: 1;
    }

    .health {
      position: absolute;
      left: 5em;
      background-color: gray;
      padding: 0.5em;
      z-index: 2;
    }

    .wave {
      position: absolute;
      left: 12em;
      background-color: gray;
      padding: 0.5em;
      z-index: 2;
    }

    .gold {
      position: absolute;
      left: 18em;
      background-color: gray;
      padding: 0.5em;
      z-index: 2;
    }

    .speed-controls {
      position: absolute;
      z-index: 2;
      bottom: 10px;
      left: 12em;
      background-color: gray;
      padding: 0.5em;
    }

    .speed-controls div {
      display: inline;
      padding: 0.5em;
    }

    .increase-speed,
    .decrease-speed {
      border: 1px solid black;
      cursor: pointer;
    }

    .start {
      position: absolute;
      font-size: 2rem;
      bottom: 10px;
      left: 2em;
      padding: 0.5em;
      cursor: pointer;
      z-index: 2;
    }

    body.fighting .start {
      background-color: gray;
    }

    body.building .start {
      background-color: green;
    }

    .result {
      z-index: 2;
      position: absolute;
      display: none;
      font-size: 3rem;
      top: 40%;
      left: 10%;
      padding: 0.5em;
    }

    .result.won {
      background-color: green;
    }

    .result.lost {
      background-color: red;
    }

    .building-sidebar {
      z-index: 2;
      display: none;
      position: absolute;
      width: 300px;
      height: 100vh;
      background-color: gray;
      top: 0px;
      right: 0px;
    }

    .towers {
      display: flex;
      flex-wrap: wrap;
      gap: 5px 0px;
      justify-content: space-evenly;
    }

    .turret {
      position: relative;
      display: inline-block;

      width: 70px;
      height: 70px;

      background-color: black;
      color: white;
      cursor: pointer;
      padding: 10px;
    }

    .cost {
      position: absolute;
      bottom: 0;
      left: 0;
      width: 25px;
      height: 25px;
      padding-top: 5px;
      text-align: center;
      color: black;
      border-radius: 50%;
      background-color: yellow;
      font-size: 0.8rem;
    }

    .close {
      background-color: rgb(24, 22, 22);
      font-size: 2rem;
      color: white;
      text-align: center;
      width: 60px;
      height: 40px;
      margin-top: 10px;
      margin-bottom: 10px;
      margin-left: -10px;
    }

    .message {
      /* display: none; */
      font-size: 1.5rem;
      margin-left: 0.5em;
      margin-top: 1em;
    }

    .tower-detail-sidebar {
      z-index: 2;
      position: fixed;
      width: 300px;
      height: 100%;
      background-color: gray;
      top: 0;
      right: 0;
    }

    .tower-stats {
      margin-left: 1em;
    }

    .tower-stats table {
      text-align: left;
    }

    .tower-stats th,
    td {
      padding-right: 0.5em;
    }

    .tower-detail-buttons {
      margin-top: 1em;
      margin-left: 1em;
    }

    .tower-upgrade {
      display: inline-block;
      font-size: 1.5rem;
      background-color: hsl(244, 60%, 45%);
      padding: 0.3em 1em;
      cursor: pointer;
    }

    .tower-sell {
      display: inline-block;
      font-size: 1.5rem;
      background-color: hsl(0, 60%, 45%);
      padding: 0.3em 1em;
      cursor: pointer;
    }
  </style>
`;

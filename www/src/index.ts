import axios from "axios";

async function main() {
  const websocket = new WebSocket("ws://localhost:8080/ws");

  websocket.onopen = () => {
    websocket.send(
      JSON.stringify({
        Register: null,
      })
    );
  };

  websocket.onmessage = (m) => {
    console.log(m);
  };
}

main();

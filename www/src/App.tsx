import React, { useEffect, useState } from "react";
import Board from "./Board";

export default function App() {
    const [playerId, setPlayerId] = useState<number>();
    const [gameId, setGameId] = useState<number>();

    useEffect(setupWebsocket, []);

    function setupWebsocket() {
        const websocket = new WebSocket("ws://localhost:8080/ws");

        websocket.onopen = () => {
            websocket.send(
                JSON.stringify({
                    Register: null,
                })
            );
        };

        websocket.onmessage = (m) => {
            const data = JSON.parse(m.data);
            if (data["Registered"] !== undefined) {
                setPlayerId(data["Registered"]["player_id"]);
                websocket.send(
                    JSON.stringify({
                        Matchup: null,
                    })
                );
            } else if (data["Matched"] !== undefined) {
                console.log(data["Matched"]);
                setGameId(data["Matched"]["game_id"]);
            }
        };
    }

    return (
        <div>
            <Board onJumpSelected={(start, end) => console.log(start, end)} />
        </div>
    );
}

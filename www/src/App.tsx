import React, { useEffect, useState } from "react";
import Board from "./Board";
import { PiecesTable, Team } from "./checkers";

export default function App() {
    let [playerId, setPlayerId] = useState<number>();
    let [websocket, setWebsocket] = useState<WebSocket>();
    const [gameId, setGameId] = useState<number>();
    const [team, setTeam] = useState<Team>("Light");
    const [table, setTable] = useState<PiecesTable>([]);
    const [teamOnTurn, setTeamOnTurn] = useState<Team>("Light");

    useEffect(setupWebsocket, []);

    function setupWebsocket() {
        const websocket = new WebSocket("ws://localhost:8080/ws");
        setWebsocket(websocket);

        websocket.onopen = () => {
            websocket.send(
                JSON.stringify({
                    Register: null,
                })
            );
        };

        websocket.onmessage = (m) => {
            console.log(m);
            const data = JSON.parse(m.data);
            if (data["Registered"] !== undefined) {
                playerId = data["Registered"]["player_id"];
                setPlayerId(playerId);
                websocket.send(
                    JSON.stringify({
                        Matchup: null,
                    })
                );
            } else if (data["Matched"] !== undefined) {
                setGameId(data["Matched"]["game_id"]);
                setTeam(
                    data["Matched"]["light_player"] === playerId
                        ? "Light"
                        : "Dark"
                );
            } else if (data["GameUpdate"] !== undefined) {
                setTable(data["GameUpdate"]["table"]);
                setTeamOnTurn(data["GameUpdate"]["team_on_turn"]);
            } else if (data["BadJump"] !== undefined) {
                console.log("Bad jump!");
            }
        };
    }

    function sendJump(from: number, to: number) {
        if (websocket === undefined) return;

        console.log(from, to);

        websocket.send(
            JSON.stringify({
                Jump: {
                    from,
                    to,
                },
            })
        );
    }

    return (
        <div>
            <Board onJumpSelected={sendJump} table={table} />

            <p>Player ID: {playerId}</p>
            <p>Game ID: {gameId}</p>
            <p>Team: {team}</p>
            <p>Team On Turn: {teamOnTurn}</p>
        </div>
    );
}

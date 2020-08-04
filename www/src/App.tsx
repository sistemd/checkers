import React, { useEffect, useState } from "react";
import Board from "./Board";
import { PiecesTable, Team } from "./checkers";

export default function App() {
    let [playerId, setPlayerId] = useState<number>();
    const [websocket, setWebsocket] = useState<WebSocket>();
    const [gameId, setGameId] = useState<number>();
    const [team, setTeam] = useState<Team>("Light");
    const [winner, setWinner] = useState<Team | null>(null);
    let [table, setTable] = useState<PiecesTable>([]);
    const [teamOnTurn, setTeamOnTurn] = useState<Team>("Light");

    useEffect(() => {
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
            } else if (data["GameFound"] !== undefined) {
                setGameId(data["GameFound"]["game_id"]);
                setTeam(
                    data["GameFound"]["light_player"] === playerId
                        ? "Light"
                        : "Dark"
                );
            } else if (data["GameState"] !== undefined) {
                table = data["GameState"][
                    "table"
                ].map((piece: any, key: number) =>
                    piece === null ? null : { key, ...piece }
                );
                setTable(table);
                setTeamOnTurn(data["GameState"]["team_on_turn"]);
                setWinner(data["GameState"]["winner"]);
            } else if (data["GameUpdate"] !== undefined) {
                const from = data["GameUpdate"]["from"];
                const to = data["GameUpdate"]["to"];
                table[to] = table[from];
                table[from] = null;
                if (data["GameUpdate"]["captured_piece"] !== null) {
                    table[data["GameUpdate"]["captured_piece"]] = null;
                }
                setTable(table);
                setTeamOnTurn(data["GameUpdate"]["team_on_turn"]);
                setWinner(data["GameUpdate"]["winner"]);
            } else if (data["BadJump"] !== undefined) {
                console.log("Bad jump!");
            }
        };
    }, []);

    function sendJump(from: number, to: number) {
        if (websocket === undefined) return;

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
            <Board onJumpSelected={sendJump} table={table} team={team} />

            <p>
                Player ID - Game ID: {playerId} - {gameId}
            </p>
            <p>
                Team - On Turn: {team} - {teamOnTurn}
            </p>
            <p>Winner: {winner}</p>
        </div>
    );
}

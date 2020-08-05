import React, { useEffect, useState } from "react";
import Board from "./Board";
import { Game, Team } from "./checkers";

export default function App() {
    let [playerId, setPlayerId] = useState<number>();
    const [websocket, setWebsocket] = useState<WebSocket>();
    const [gameId, setGameId] = useState<number>();
    const [team, setTeam] = useState<Team>("Light");
    const [winner, setWinner] = useState<Team | null>(null);
    let [game, setGame] = useState<Game>();
    const [teamOnTurn, setTeamOnTurn] = useState<Team>("Light");

    useEffect(() => {
        const websocket = new WebSocket(`ws://${window.location.host}/ws`);
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
                game = new Game(data["GameState"]["table"]);
                setGame(game);
                setTeamOnTurn(data["GameState"]["team_on_turn"]);
                setWinner(data["GameState"]["winner"]);
            } else if (data["GameUpdate"] !== undefined) {
                if (game === undefined) {
                    throw new Error(
                        "trying to update, but pieces is undefined"
                    );
                }

                const from = data["GameUpdate"]["from"];
                const to = data["GameUpdate"]["to"];
                const captured = data["GameUpdate"]["captured_piece"];
                const crowned = data["GameUpdate"]["crowned"];
                game.update(from, to, captured, crowned);
                setGame(game);
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
            {game !== undefined && (
                <Board onJumpSelected={sendJump} game={game} team={team} />
            )}
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

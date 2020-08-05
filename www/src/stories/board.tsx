import React, { useState } from "react";
import Board from "../Board";
import { boardSize } from "../Fields";
import _ from "lodash";
import { Game } from "../checkers";
import { action } from "@storybook/addon-actions";

export default {
    title: "Board",
    component: Board,
};

export const boardForLightPlayer = () => (
    <Board
        game={new Game(samplePiecesTable())}
        team="Light"
        onJumpSelected={(start, end) => action(`jumpSelected ${start} ${end}`)}
    />
);

export const boardForDarkPlayer = () => (
    <Board
        game={new Game(samplePiecesTable())}
        team="Dark"
        onJumpSelected={(start, end) => action(`jumpSelected ${start} ${end}`)}
    />
);

export function movingAllPiecesFreely() {
    const [game, setGame] = useState(new Game(samplePiecesTable()));

    function onJumpSelected(from: number, to: number) {
        console.log(game.pieces);
        game.update(from, to);
        console.log(game.pieces);
        setGame(game);
    }

    return <Board game={game} team="Dark" onJumpSelected={onJumpSelected} />;
}

function samplePiecesTable() {
    const table: any = _.range(boardSize / 2).map(() => null);
    table[0] = { kind: "Man", team: "Light" };
    table[1] = { kind: "Man", team: "Light" };
    table[5] = { kind: "Man", team: "Light" };
    table[6] = { kind: "King", team: "Dark" };
    table[25] = { kind: "Man", team: "Dark" };
    table[26] = { kind: "Man", team: "Dark" };
    table[15] = { kind: "King", team: "Light" };
    return table;
}

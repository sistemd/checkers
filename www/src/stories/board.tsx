import React from "react";
import Board from "../Board";
import { boardSize } from "../Fields";
import _ from "lodash";
import { PiecesTable } from "../checkers";
import { action } from "@storybook/addon-actions";

export default {
    title: "Board",
    component: Board,
};

export const boardForLightPlayer = () => (
    <Board
        table={samplePiecesTable()}
        team="Light"
        onJumpSelected={(start, end) => action(`jumpSelected ${start} ${end}`)}
    />
);

export const boardForDarkPlayer = () => (
    <Board
        table={samplePiecesTable()}
        team="Dark"
        onJumpSelected={(start, end) => action(`jumpSelected ${start} ${end}`)}
    />
);

function samplePiecesTable() {
    const table: PiecesTable = _.range(boardSize / 2).map(() => null);
    table[0] = table[1] = table[5] = { kind: "Man", team: "Light" };
    table[6] = { kind: "King", team: "Dark" };
    table[25] = table[26] = { kind: "Man", team: "Dark" };
    table[15] = { kind: "King", team: "Light" };
    return table;
}

import React, { useState } from "react";
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

export function movingAllPiecesFreely() {
    const [table, setTable] = useState(samplePiecesTable());

    function onJumpSelected(start: number, end: number) {
        table[end] = table[start];
        table[start] = null;
        setTable(table);
    }

    return <Board table={table} team="Dark" onJumpSelected={onJumpSelected} />;
}

function samplePiecesTable() {
    const table: PiecesTable = _.range(boardSize / 2).map(() => null);
    table[0] = { kind: "Man", team: "Light", key: 0 };
    table[1] = { kind: "Man", team: "Light", key: 1 };
    table[5] = { kind: "Man", team: "Light", key: 2 };
    table[6] = { kind: "King", team: "Dark", key: 3 };
    table[25] = { kind: "Man", team: "Dark", key: 4 };
    table[26] = { kind: "Man", team: "Dark", key: 5 };
    table[15] = { kind: "King", team: "Light", key: 6 };
    return table;
}

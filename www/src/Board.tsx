import React, { useState } from "react";
import Fields, { boardWidth, boardHeight } from "./Fields";
import { PiecesTable, Team } from "./checkers";
import Pieces from "./Pieces";

export interface BoardProps {
    onJumpSelected(start: number, end: number): void;
    table: PiecesTable;
    team: Team;
}

export default function Board(props: BoardProps) {
    const [selectedIndex, setSelectedIndex] = useState<number>();

    function onFieldClicked(fieldIndex: number) {
        console.log(fieldIndex);
        if (selectedIndex === undefined) {
            setSelectedIndex(fieldIndex);
        } else {
            props.onJumpSelected(selectedIndex, fieldIndex);
            setSelectedIndex(undefined);
        }
    }

    const transform = props.team === "Light" ? "scale(1, -1)" : "";

    return (
        <svg
            viewBox={`0 0 ${boardWidth} ${boardHeight}`}
            style={{ width: boardWidth }}
            transform={transform}
        >
            <Fields onFieldClicked={onFieldClicked} />
            <Pieces
                selectedIndex={selectedIndex}
                onPieceClicked={onFieldClicked}
                table={props.table}
                flipCrowns={props.team === "Light"}
            />
        </svg>
    );
}

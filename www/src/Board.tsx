import React, { useState } from "react";
import Fields, { boardWidth, boardHeight } from "./Fields";
import { PiecesTable } from "./checkers";
import Pieces from "./Pieces";

export interface BoardProps {
    onJumpSelected(start: number, end: number): void;
    table: PiecesTable;
}

export default function Board(props: BoardProps) {
    const [start, setStart] = useState<number>();

    function onFieldClicked(fieldIndex: number) {
        console.log(fieldIndex);
        if (start === undefined) {
            setStart(fieldIndex);
        } else {
            props.onJumpSelected(start, fieldIndex);
            setStart(undefined);
        }
    }

    return (
        <svg
            viewBox={`0 0 ${boardWidth} ${boardHeight}`}
            style={{ width: boardWidth }}
        >
            <Fields onFieldClicked={onFieldClicked} />
            <Pieces onPieceClicked={onFieldClicked} table={props.table} />
        </svg>
    );
}

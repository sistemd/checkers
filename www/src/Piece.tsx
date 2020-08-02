import React from "react";
import { Piece as CheckersPiece } from "./checkers";
import { fieldWidth, fieldHeight } from "./Field";

export interface PieceProps {
    onClick(): void;
    piece: CheckersPiece | null;
    index: number;
}

export default function Piece(props: PieceProps) {
    if (props.piece === null) return null;

    const radius = fieldWidth / 3;
    const row = Math.floor(props.index / 4);
    const column =
        row % 2 == 0 ? (props.index % 4) * 2 + 1 : (props.index % 4) * 2;
    const fill = props.piece.team === "Light" ? "red" : "blue";
    return (
        <circle
            cx={column * fieldWidth + fieldWidth / 2}
            cy={row * fieldHeight + fieldHeight / 2}
            r={radius}
            fill={fill}
            onClick={props.onClick}
        ></circle>
    );
}

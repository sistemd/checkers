import React from "react";
import { Piece as CheckersPiece } from "./checkers";
import { fieldWidth, fieldHeight } from "./Field";
import Crown from "./Crown";
import {
    lightPieceColor,
    darkPieceColor,
    selectedOutlineColor,
} from "./colors";
import "./animations.css";

export interface PieceProps {
    onClick(): void;
    piece: CheckersPiece;
    selected: boolean;
    flipCrown?: boolean;
}

export default function Piece(props: PieceProps) {
    const radius = props.piece.dead ? 0 : fieldWidth / 3;
    const row = Math.floor(props.piece.pos / 4);
    const column =
        row % 2 == 0
            ? (props.piece.pos % 4) * 2 + 1
            : (props.piece.pos % 4) * 2;
    const fill =
        props.piece.team === "Light" ? lightPieceColor : darkPieceColor;
    const stroke = props.selected ? selectedOutlineColor : "";
    const crownOpacity =
        !props.piece.dead && props.piece.kind === "King" ? 1 : 0;

    return (
        <>
            <circle
                className="animate"
                cx={column * fieldWidth + fieldWidth / 2}
                cy={row * fieldHeight + fieldHeight / 2}
                r={radius}
                fill={fill}
                stroke={stroke}
                strokeWidth="5"
                onClick={props.onClick}
            />
            <Crown
                opacity={crownOpacity}
                row={row}
                column={column}
                flip={props.flipCrown}
                onClick={props.onClick}
            />
        </>
    );
}

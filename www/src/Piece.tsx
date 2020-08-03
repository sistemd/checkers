import React from "react";
import { Piece as CheckersPiece } from "./checkers";
import { fieldWidth, fieldHeight } from "./Field";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faCrown, faCrow } from "@fortawesome/free-solid-svg-icons";

const lightColor = "red";
const darkColor = "blue";
const selectedColor = "black";

export interface PieceProps {
    onClick(): void;
    piece: CheckersPiece | null;
    index: number;
    selected: boolean;
    flip?: boolean;
}

export default function Piece(props: PieceProps) {
    if (props.piece === null) return null;

    const radius = fieldWidth / 3;
    const row = Math.floor(props.index / 4);
    const column =
        row % 2 == 0 ? (props.index % 4) * 2 + 1 : (props.index % 4) * 2;
    const fill = props.piece.team === "Light" ? lightColor : darkColor;
    const stroke = props.selected ? selectedColor : "";

    const crownWidth = fieldWidth - 5;
    const crownHeight = fieldHeight;

    return (
        <>
            <circle
                cx={column * fieldWidth + fieldWidth / 2}
                cy={row * fieldHeight + fieldHeight / 2}
                r={radius}
                fill={fill}
                stroke={stroke}
                strokeWidth="5"
                onClick={props.onClick}
            />
            {props.piece.kind === "King" && (
                <svg
                    transform={
                        `translate(` +
                        `${column * fieldWidth}, ${row * fieldHeight})` +
                        (props.flip
                            ? `scale(1, -1) translate(0, ${-fieldHeight})`
                            : "") +
                        `scale(` +
                        `${crownWidth / 512}, ${crownHeight / 512}) ` +
                        "scale(0.7)"
                    }
                >
                    <FontAwesomeIcon
                        icon={faCrown}
                        color="grey"
                        style={{ width: fieldWidth, height: fieldHeight }}
                    />
                </svg>
            )}
        </>
    );
}

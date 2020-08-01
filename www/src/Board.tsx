import React, { useState } from "react";
import _ from "lodash";

export interface BoardProps {
    onJumpSelected(start: number, end: number): void;
}

export default function Board(props: BoardProps) {
    const [start, setStart] = useState<number>();

    function onFieldClicked(fieldIndex: number) {
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
        </svg>
    );
}

const boardSize = 8;
const fieldWidth = 64;
const fieldHeight = 64;
const boardWidth = boardSize * fieldWidth;
const boardHeight = boardSize * fieldHeight;

function Fields(props: { onFieldClicked(fieldIndex: number): void }) {
    const fields = _.range(boardSize * boardSize).map((i) => {
        const row = Math.floor(i / boardSize);
        const column = i % boardSize;
        const isDark =
            (i % 2 === 0 && row % 2 === 0) || (i % 2 === 1 && row % 2 === 1);

        if (isDark) {
            return <Field key={i} fill="black" row={row} column={column} />;
        } else {
            const fieldIndex = row % 2 === 0 ? (i - 1) / 2 : i / 2;
            return (
                <Field
                    key={i}
                    fill="white"
                    row={row}
                    column={column}
                    onClick={() => props.onFieldClicked(fieldIndex)}
                />
            );
        }
    });

    return <>{fields}</>;
}

function Field(props: {
    onClick?(): void;
    row: number;
    column: number;
    fill: string;
}) {
    return (
        <rect
            y={props.row * fieldHeight}
            x={props.column * fieldWidth}
            fill={props.fill}
            onClick={props.onClick}
            width={fieldWidth}
            height={fieldHeight}
        />
    );
}

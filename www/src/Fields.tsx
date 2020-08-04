import React from "react";
import Field, { fieldWidth, fieldHeight } from "./Field";
import _ from "lodash";
import { darkFieldColor, lightFieldColor } from "./colors";

export const boardSize = 8;
export const boardWidth = boardSize * fieldWidth;
export const boardHeight = boardSize * fieldHeight;

export interface FieldsProps {
    onFieldClicked(fieldIndex: number): void;
}

export default function Fields(props: FieldsProps) {
    const fields = _.range(boardSize * boardSize).map((i) => {
        const row = Math.floor(i / boardSize);
        const column = i % boardSize;
        const isDark =
            (i % 2 === 0 && row % 2 === 0) || (i % 2 === 1 && row % 2 === 1);

        if (isDark) {
            return (
                <Field
                    key={i}
                    fill={darkFieldColor}
                    row={row}
                    column={column}
                />
            );
        } else {
            const fieldIndex = row % 2 === 0 ? (i - 1) / 2 : i / 2;
            return (
                <Field
                    key={i}
                    fill={lightFieldColor}
                    row={row}
                    column={column}
                    onClick={() => props.onFieldClicked(fieldIndex)}
                />
            );
        }
    });

    return <>{fields}</>;
}

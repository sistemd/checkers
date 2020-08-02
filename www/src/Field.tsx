import React from "react";

export const fieldWidth = 64;
export const fieldHeight = 64;

export interface FieldProps {
    onClick?(): void;
    row: number;
    column: number;
    fill: string;
}

export default function Field(props: FieldProps) {
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

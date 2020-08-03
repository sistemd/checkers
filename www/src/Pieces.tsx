import React from "react";
import { PiecesTable } from "./checkers";
import Piece from "./Piece";

export interface PiecesProps {
    onPieceClicked(index: number): void;
    table: PiecesTable;
    selectedIndex?: number;
}

export default function Pieces(props: PiecesProps) {
    const pieces = props.table.map((piece, index) => (
        <Piece
            onClick={() => props.onPieceClicked(index)}
            key={index}
            piece={piece}
            index={index}
            stroke-width="15"
            selected={index === props.selectedIndex}
        />
    ));

    return <>{pieces}</>;
}

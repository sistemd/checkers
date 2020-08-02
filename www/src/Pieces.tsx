import React from "react";
import { PiecesTable } from "./checkers";
import Piece from "./Piece";

export interface PiecesProps {
    onPieceClicked(index: number): void;
    table: PiecesTable;
}

export default function Pieces(props: PiecesProps) {
    const pieces = props.table.map((piece, index) => (
        <Piece
            onClick={() => props.onPieceClicked(index)}
            key={index}
            piece={piece}
            index={index}
        />
    ));

    return <>{pieces}</>;
}

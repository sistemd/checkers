import React from "react";
import { PiecesTable } from "./checkers";
import Piece from "./Piece";

export interface PiecesProps {
    onPieceClicked(index: number): void;
    table: PiecesTable;
    selectedIndex?: number;
    flipCrowns?: boolean;
}

export default function Pieces(props: PiecesProps) {
    console.log(props.table);
    const pieces = props.table.map((piece, index) =>
        piece === null ? null : (
            <Piece
                onClick={() => props.onPieceClicked(index)}
                key={piece.key}
                piece={piece}
                index={index}
                selected={index === props.selectedIndex}
                flipCrown={props.flipCrowns}
            />
        )
    );

    return <>{pieces}</>;
}

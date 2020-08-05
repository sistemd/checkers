import React from "react";
import { Piece as CheckersPiece } from "./checkers";
import Piece from "./Piece";
import _ from "lodash";

export interface PiecesProps {
    onPieceClicked(index: number): void;
    pieces: CheckersPiece[];
    selectedPos?: number;
    flipCrowns?: boolean;
}

export default function Pieces(props: PiecesProps) {
    let pieces = props.pieces.map((piece, index) => (
        <Piece
            onClick={() => props.onPieceClicked(piece.pos)}
            key={index}
            piece={piece}
            selected={piece.pos === props.selectedPos}
            flipCrown={props.flipCrowns}
        />
    ));

    return <>{pieces}</>;
}

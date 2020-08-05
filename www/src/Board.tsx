import React, { useState, useEffect } from "react";
import Fields, { boardWidth, boardHeight } from "./Fields";
import { Team, Game } from "./checkers";
import Pieces from "./Pieces";

export interface BoardProps {
    onJumpSelected(start: number, end: number): void;
    game: Game;
    team: Team;
}

export default function Board(props: BoardProps) {
    const [selectedPos, setSelectedPos] = useState<number>();
    const [size, setSize] = useState(calculateSize());

    useEffect(() => {
        window.onresize = () => setSize(calculateSize());
    }, []);

    function calculateSize() {
        return Math.min(window.innerWidth, window.innerHeight);
    }

    function onFieldClicked(fieldIndex: number) {
        console.log(fieldIndex);
        if (selectedPos === undefined) {
            setSelectedPos(fieldIndex);
        } else {
            props.onJumpSelected(selectedPos, fieldIndex);
            setSelectedPos(undefined);
        }
    }

    const transform = props.team === "Light" ? "scale(1, -1)" : "";

    return (
        <svg
            viewBox={`0 0 ${boardWidth} ${boardHeight}`}
            style={{ width: size }}
            transform={transform}
        >
            <Fields onFieldClicked={onFieldClicked} />
            <Pieces
                selectedPos={selectedPos}
                onPieceClicked={onFieldClicked}
                pieces={props.game.pieces}
                flipCrowns={props.team === "Light"}
            />
        </svg>
    );
}

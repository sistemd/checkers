import React from "react";
import { action } from "@storybook/addon-actions";
import Piece from "../Piece";
import { boardWidth, boardHeight } from "../Fields";

export default {
    title: "Piece",
    component: Piece,
};

export const lightMan = () => (
    <svg
        viewBox={`0 0 ${boardWidth} ${boardHeight}`}
        style={{ width: boardWidth, height: boardHeight }}
    >
        <Piece
            onClick={() => action("onClick")}
            piece={{ kind: "Man", team: "Light", dead: false, pos: 0 }}
            selected={false}
        />
    </svg>
);

export const darkMan = () => (
    <svg
        viewBox={`0 0 ${boardWidth} ${boardHeight}`}
        style={{ width: boardWidth, height: boardHeight }}
    >
        <Piece
            onClick={() => action("onClick")}
            piece={{ kind: "Man", team: "Dark", dead: false, pos: 0 }}
            selected={false}
        />
    </svg>
);

export const lightKing = () => (
    <svg
        viewBox={`0 0 ${boardWidth} ${boardHeight}`}
        style={{ width: boardWidth, height: boardHeight }}
    >
        <Piece
            onClick={() => action("onClick")}
            piece={{ kind: "King", team: "Light", dead: false, pos: 0 }}
            selected={false}
        />
    </svg>
);

export const darkKing = () => (
    <svg
        viewBox={`0 0 ${boardWidth} ${boardHeight}`}
        style={{ width: boardWidth, height: boardHeight }}
    >
        <Piece
            onClick={() => action("onClick")}
            piece={{ kind: "King", team: "Dark", dead: false, pos: 0 }}
            selected={false}
        />
    </svg>
);

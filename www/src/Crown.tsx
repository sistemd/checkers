import React from "react";
import { boardWidth, boardHeight } from "./Fields";
import { fieldWidth, fieldHeight } from "./Field";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faCrown } from "@fortawesome/free-solid-svg-icons";
import "./animations.css";

export interface CrownProps {
    onClick?(): void;
    column: number;
    row: number;
    flip?: boolean;
}

export default function Crown(props: CrownProps) {
    const crownWidth = fieldWidth / 2;
    const crownHeight = (fieldHeight - 5) / 2;

    const crownTransform = [
        `translate(${-boardWidth / 2}px, ${-boardHeight / 2}px)`, // Move the crown to bottom left corner
        `translate(${fieldWidth / 2}px, ${fieldHeight / 2}px)`, // Center the crown
        `translate(${props.column * fieldWidth}px, ${
            props.row * fieldHeight
        }px)`, // Move the crown on top of the piece
        props.flip ? "scale(1, -1)" : "",
        "translate(0, -1px)", // Polish
        `scale(${crownWidth / 640}, ${crownHeight / 512})`,
    ].join(" ");

    return (
        <svg
            className="animate-position"
            onClick={props.onClick}
            style={{
                transformOrigin: "center",
                transform: crownTransform,
            }}
        >
            <FontAwesomeIcon
                icon={faCrown}
                color="grey"
                style={{ width: fieldWidth, height: fieldHeight }}
            />
        </svg>
    );
}

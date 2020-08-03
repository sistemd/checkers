import React from "react";
import Fields, { boardWidth, boardHeight } from "../Fields";
import { action } from "@storybook/addon-actions";

export default {
    title: "Fields",
    component: Fields,
};

export const fields = () => (
    <svg
        viewBox={`0 0 ${boardWidth} ${boardHeight}`}
        style={{ width: boardWidth, height: boardHeight }}
    >
        <Fields onFieldClicked={() => action("onFieldClicked")} />
    </svg>
);

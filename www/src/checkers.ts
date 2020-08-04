export interface Piece {
    key: number; // Unique key, used for drawing
    team: Team;
    kind: PieceKind;
}

export type Team = "Light" | "Dark";

export type PieceKind = "Man" | "King";

export type PiecesTable = (Piece | null)[];

import _ from "lodash";

export interface Piece {
    dead: boolean;
    pos: number;
    team: Team;
    kind: PieceKind;
}

export type Team = "Light" | "Dark";

export type PieceKind = "Man" | "King";

export class Game {
    pieces: Piece[];

    constructor(table: any) {
        this.pieces = table
            .map((piece: any, pos: number) =>
                piece
                    ? {
                          dead: false,
                          pos,
                          ...piece,
                      }
                    : null
            )
            .filter((p: any) => p !== null);
    }

    update(
        from: number,
        to: number,
        captured?: number | null,
        crowned?: boolean
    ) {
        const fromPiece = this.pieceAt(from);
        if (fromPiece === undefined)
            throw new Error("moved piece is undefined");

        fromPiece.pos = to;
        if (crowned) fromPiece.kind = "King";

        const capturedPiece = this.pieceAt(captured);
        if (capturedPiece !== undefined) capturedPiece.dead = true;
    }

    private pieceAt(pos?: number | null): Piece | undefined {
        if (pos === undefined || pos === null) return undefined;
        return _.find(this.pieces, (p) => !p.dead && p.pos === pos);
    }
}

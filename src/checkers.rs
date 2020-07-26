use std::collections::HashSet;
use std::iter::FromIterator;

pub struct CheckersGame {
    table: Table,
    team_on_turn: Team,
}

const TABLE_ROWS: usize = 8;
const TABLE_COLUMNS: usize = 8;
const FIELDS_PER_ROW: usize = TABLE_COLUMNS / 2;
const TABLE_SIZE: usize = TABLE_ROWS * TABLE_COLUMNS / 2;

pub type Table = [Option<Piece>; TABLE_SIZE];

#[derive(Copy, Clone)]
pub struct Piece {
    pub team: Team,
    pub kind: PieceKind,
}

#[derive(Copy, Clone)]
pub enum PieceKind {
    Man,
    King,
}

impl Piece {
    const LIGHT_MAN: Piece = Piece {
        team: Team::Light,
        kind: PieceKind::Man,
    };

    const LIGHT_KING: Piece = Piece {
        team: Team::Light,
        kind: PieceKind::King,
    };

    const DARK_MAN: Piece = Piece {
        team: Team::Dark,
        kind: PieceKind::Man,
    };

    const DARK_KING: Piece = Piece {
        team: Team::Dark,
        kind: PieceKind::King,
    };
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Team {
    Light,
    Dark,
}

impl Team {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }
}

impl CheckersGame {
    fn from_table(table: Table) -> Self {
        Self {
            table,
            team_on_turn: Team::Light,
        }
    }

    pub fn new() -> Self {
        let mut table = [None; TABLE_SIZE];
        for i in 0..12 {
            table[i] = Some(Piece::LIGHT_MAN);
        }
        for i in 20..32 {
            table[i] = Some(Piece::DARK_MAN);
        }
        Self::from_table(table)
    }

    /// Returns the positions of those pieces which must capture on next move.
    pub fn capturing_pieces(&self) -> Vec<usize> {
        self.table
            .iter()
            .enumerate()
            .map(|(pos, piece)| match piece {
                Some(piece) => Self::advance_positions(pos, *piece)
                    .into_iter()
                    .filter(move |&i| match self.table[i] {
                        Some(other_piece) => piece.team != other_piece.team,
                        None => false,
                    })
                    .collect(),
                None => vec![],
            })
            .flatten()
            .collect()
    }

    fn table_row(pos: usize) -> usize {
        pos / 4
    }

    fn adjacent_table_positions(pos: usize) -> Vec<usize> {
        let row = Self::table_row(pos) as i32;
        let offset: i32 = if row % 2 == 0 {
            FIELDS_PER_ROW as i32
        } else {
            FIELDS_PER_ROW as i32 + 1
        };

        [
            pos as i32 - offset,
            pos as i32 - offset + 1,
            pos as i32 - offset + TABLE_COLUMNS as i32,
            pos as i32 - offset + TABLE_COLUMNS as i32 + 1,
        ]
        .iter()
        .filter(move |&p| {
            0 <= *p
                && *p < TABLE_SIZE as i32
                && (Self::table_row(*p as usize) as i32 - row).abs() == 1
        })
        .map(|&p| p as usize)
        .collect()
    }

    fn advance_positions(pos: usize, piece: Piece) -> Vec<usize> {
        match piece.kind {
            PieceKind::King => Self::adjacent_table_positions(pos),
            PieceKind::Man => Self::adjacent_table_positions(pos)
                .into_iter()
                .filter(|p| {
                    let row_offset = match piece.team {
                        Team::Light => 1,
                        Team::Dark => -1,
                    };

                    Self::table_row(*p) as i32 - Self::table_row(pos) as i32 == row_offset
                })
                .collect(),
        }
    }

    /// Returns the team which is currently on turn.
    pub fn team_on_turn(&self) -> Team {
        self.team_on_turn
    }

    fn toggle_team_on_turn(&mut self) {
        self.team_on_turn = self.team_on_turn.opposite();
    }

    /// Jump piece at position from to position to.
    /// Returns true if the jump was successful, false otherwise.
    pub fn jump(&mut self, from: usize, to: usize) -> bool {
        self.table[to] = self.table[from];
        self.table[from] = None;
        true
    }

    pub fn table(&self) -> &Table {
        &self.table
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjacent_table_positions() {
        assert_eq!(CheckersGame::adjacent_table_positions(0), vec![4, 5]);
        assert_eq!(CheckersGame::adjacent_table_positions(12), vec![8, 16]);
        assert_eq!(CheckersGame::adjacent_table_positions(11), vec![7, 15]);
        assert_eq!(CheckersGame::adjacent_table_positions(27), vec![23, 31]);
        assert_eq!(CheckersGame::adjacent_table_positions(28), vec![24]);
        assert_eq!(
            CheckersGame::adjacent_table_positions(9),
            vec![5, 6, 13, 14]
        );
    }

    #[test]
    fn advance_positions() {
        assert_eq!(
            CheckersGame::advance_positions(9, Piece::LIGHT_KING),
            vec![5, 6, 13, 14]
        );
        assert_eq!(
            CheckersGame::advance_positions(9, Piece::DARK_KING),
            vec![5, 6, 13, 14]
        );
        assert_eq!(
            CheckersGame::advance_positions(9, Piece::LIGHT_MAN),
            vec![13, 14]
        );
        assert_eq!(
            CheckersGame::advance_positions(9, Piece::DARK_MAN),
            vec![5, 6]
        );
    }

    #[test]
    fn capturing_pieces() {
        let game = CheckersGame::from_table([None; TABLE_SIZE]);
        assert_eq!(game.capturing_pieces(), Vec::<usize>::new());

        let mut table = [None; TABLE_SIZE];
        table[5] = Some(Piece::LIGHT_MAN);
        table[6] = Some(Piece::LIGHT_MAN);
        table[9] = Some(Piece::LIGHT_MAN);
        table[13] = Some(Piece::LIGHT_MAN);
        table[14] = Some(Piece::LIGHT_MAN);
        table[4] = Some(Piece::DARK_MAN);
        let game = CheckersGame::from_table(table);
        assert_eq!(game.capturing_pieces(), Vec::<usize>::new());

        let mut table = [None; TABLE_SIZE];
        table[5] = Some(Piece::DARK_MAN);
        table[6] = Some(Piece::LIGHT_MAN);
        table[9] = Some(Piece::LIGHT_MAN);
        table[13] = Some(Piece::LIGHT_MAN);
        table[14] = Some(Piece::LIGHT_MAN);
        table[4] = Some(Piece::DARK_MAN);
        let mut game = CheckersGame::from_table(table);
        assert_eq!(game.capturing_pieces(), Vec::<usize>::new());
        game.toggle_team_on_turn();
        assert_eq!(game.capturing_pieces(), Vec::<usize>::new());

        let mut table = [None; TABLE_SIZE];
        table[5] = Some(Piece::LIGHT_MAN);
        table[6] = Some(Piece::LIGHT_MAN);
        table[9] = Some(Piece::LIGHT_MAN);
        table[13] = Some(Piece::LIGHT_MAN);
        table[14] = Some(Piece::DARK_MAN);
        table[4] = Some(Piece::DARK_MAN);
        let game = CheckersGame::from_table(table);
        assert_eq!(
            HashSet::<_>::from_iter(game.capturing_pieces().into_iter()),
            HashSet::<_>::from_iter(vec![9, 14].into_iter())
        );

        let mut table = [None; TABLE_SIZE];
        table[5] = Some(Piece::LIGHT_MAN);
        table[6] = Some(Piece::LIGHT_MAN);
        table[8] = Some(Piece::DARK_MAN);
        table[9] = Some(Piece::LIGHT_MAN);
        table[13] = Some(Piece::LIGHT_MAN);
        table[14] = Some(Piece::DARK_MAN);
        table[4] = Some(Piece::DARK_MAN);
        let game = CheckersGame::from_table(table);
        assert_eq!(
            HashSet::<_>::from_iter(game.capturing_pieces().into_iter()),
            HashSet::<_>::from_iter(vec![9, 14, 5, 8].into_iter())
        );
    }
}

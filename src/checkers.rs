use std::collections::HashSet;
use std::iter::FromIterator;

const TABLE_ROWS: usize = 8;
const TABLE_COLUMNS: usize = 8;
const FIELDS_PER_ROW: usize = TABLE_COLUMNS / 2;
const TABLE_SIZE: usize = TABLE_ROWS * TABLE_COLUMNS / 2;

pub type Table = [Option<Piece>; TABLE_SIZE];

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Piece {
    pub team: Team,
    pub kind: PieceKind,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
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

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
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

pub struct CheckersGame {
    table: Table,
    team_on_turn: Team,
    mandatory_capturing_piece: Option<(usize, Piece)>,
}

impl CheckersGame {
    fn from_table(table: Table) -> Self {
        Self {
            table,
            team_on_turn: Team::Light,
            mandatory_capturing_piece: None,
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
    pub fn mandatory_capturing_pieces(&self) -> Vec<(usize, Piece)> {
        match self.mandatory_capturing_piece {
            Some(piece) => vec![piece],
            None => self
                .capturing_pieces()
                .into_iter()
                .filter(|(_, p)| p.team == self.team_on_turn)
                .collect(),
        }
    }

    fn capturing_pieces(&self) -> Vec<(usize, Piece)> {
        self.table
            .iter()
            .enumerate()
            .filter_map(|(pos, piece)| {
                piece.and_then(|piece| {
                    if self.is_capturing_piece(pos, piece) {
                        Some((pos, piece))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    fn is_capturing_piece(&self, pos: usize, piece: Piece) -> bool {
        Self::capture_positions(pos, piece).into_iter().any(|p| {
            let (_, captured_piece) = self.captured_piece(pos, p);
            match captured_piece {
                Some(captured_piece) => {
                    captured_piece.team != piece.team && self.table[p].is_none()
                }
                None => false,
            }
        })
    }

    fn table_row(pos: i32) -> i32 {
        pos / 4
    }

    fn adjacent_positions(pos: usize, piece: Piece) -> Vec<usize> {
        let row = Self::table_row(pos as i32);
        let offset: i32 = if row % 2 == 0 {
            FIELDS_PER_ROW as i32
        } else {
            FIELDS_PER_ROW as i32 + 1
        };

        Self::valid_advance_positions(
            [
                pos as i32 - offset,
                pos as i32 - offset + 1,
                pos as i32 - offset + TABLE_COLUMNS as i32,
                pos as i32 - offset + TABLE_COLUMNS as i32 + 1,
            ]
            .iter(),
            pos,
            piece,
            1,
        )
    }

    fn capture_positions(pos: usize, piece: Piece) -> Vec<usize> {
        let row = Self::table_row(pos as i32);
        let offset = (FIELDS_PER_ROW * 2) as i32;

        Self::valid_advance_positions(
            [
                pos as i32 - offset - 1,
                pos as i32 - offset + 1,
                pos as i32 + offset - 1,
                pos as i32 + offset + 1,
            ]
            .iter(),
            pos,
            piece,
            2,
        )
    }

    fn valid_advance_positions<'a>(
        positions: impl Iterator<Item = &'a i32>,
        pos: usize,
        piece: Piece,
        row_offset: usize,
    ) -> Vec<usize> {
        let row_offset = match piece.team {
            Team::Light => row_offset as i32,
            Team::Dark => -(row_offset as i32),
        };

        positions
            .filter(|&p| {
                if !Self::position_is_valid(*p) {
                    false
                } else {
                    match piece.kind {
                        PieceKind::King => {
                            (Self::table_row(*p) - Self::table_row(pos as i32)).abs()
                                == row_offset.abs()
                        }
                        PieceKind::Man => {
                            Self::table_row(*p) - Self::table_row(pos as i32) == row_offset
                        }
                    }
                }
            })
            .map(|&p| p as usize)
            .collect()
    }

    fn position_is_valid(pos: i32) -> bool {
        0 <= pos && pos < TABLE_SIZE as i32
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
        let piece = match self.table[from] {
            Some(piece) => piece,
            None => return false,
        };

        if piece.team != self.team_on_turn {
            return false;
        }

        if !self.field_is_free(to) {
            return false;
        }

        let mandatory_capturing_pieces: Vec<_> = self.mandatory_capturing_pieces();
        let must_capture = !mandatory_capturing_pieces.is_empty();
        if must_capture
            && !mandatory_capturing_pieces
                .into_iter()
                .any(|(pos, _)| pos == from)
        {
            return false;
        }

        if !must_capture && Self::adjacent_positions(from, piece).contains(&to) {
            self.do_jump(from, to);
            self.mandatory_capturing_piece = None;
            self.toggle_team_on_turn();
            true
        } else if Self::capture_positions(from, piece).contains(&to) {
            let (captured_pos, captured_piece) = self.captured_piece(from, to);
            match captured_piece {
                Some(captured_piece) if captured_piece.team != piece.team => {
                    self.do_jump(from, to);
                    self.table[captured_pos] = None;
                    if self.is_capturing_piece(to, piece) {
                        self.mandatory_capturing_piece = Some((to, piece));
                    } else {
                        self.mandatory_capturing_piece = None;
                        self.toggle_team_on_turn();
                    }
                    true
                }
                _ => false,
            }
        } else {
            false
        }
    }

    fn do_jump(&mut self, from: usize, to: usize) {
        self.table[to] = self.table[from];
        self.table[from] = None;
    }

    fn captured_piece(&self, from: usize, to: usize) -> (usize, Option<Piece>) {
        let captured_pos = if Self::table_row(from as i32) % 2 == 0 {
            (from + to + 1) / 2
        } else {
            (from + to) / 2
        };

        (captured_pos, self.table[captured_pos])
    }

    fn field_is_free(&self, pos: usize) -> bool {
        self.table[pos].is_none()
    }

    pub fn table(&self) -> &Table {
        &self.table
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adjacent_positions() {
        assert_eq!(
            CheckersGame::adjacent_positions(9, Piece::LIGHT_KING),
            vec![5, 6, 13, 14]
        );
        assert_eq!(
            CheckersGame::adjacent_positions(9, Piece::DARK_KING),
            vec![5, 6, 13, 14]
        );
        assert_eq!(
            CheckersGame::adjacent_positions(9, Piece::LIGHT_MAN),
            vec![13, 14]
        );
        assert_eq!(
            CheckersGame::adjacent_positions(9, Piece::DARK_MAN),
            vec![5, 6]
        );

        assert_eq!(
            CheckersGame::adjacent_positions(12, Piece::LIGHT_KING),
            vec![8, 16]
        );
        assert_eq!(
            CheckersGame::adjacent_positions(12, Piece::DARK_KING),
            vec![8, 16]
        );
        assert_eq!(
            CheckersGame::adjacent_positions(12, Piece::LIGHT_MAN),
            vec![16]
        );
        assert_eq!(
            CheckersGame::adjacent_positions(12, Piece::DARK_MAN),
            vec![8]
        );
    }

    #[test]
    fn capturing_pieces() {
        let no_capturing_pieces = Vec::<(usize, Piece)>::new();

        let game = CheckersGame::from_table([None; TABLE_SIZE]);
        assert_eq!(game.capturing_pieces(), no_capturing_pieces);

        let mut table = [None; TABLE_SIZE];
        table[5] = Some(Piece::LIGHT_MAN);
        table[6] = Some(Piece::LIGHT_MAN);
        table[9] = Some(Piece::LIGHT_MAN);
        table[13] = Some(Piece::LIGHT_MAN);
        table[14] = Some(Piece::LIGHT_MAN);
        table[4] = Some(Piece::DARK_MAN);
        let game = CheckersGame::from_table(table);
        assert_eq!(game.capturing_pieces(), no_capturing_pieces);

        let mut table = [None; TABLE_SIZE];
        table[5] = Some(Piece::DARK_MAN);
        table[6] = Some(Piece::LIGHT_MAN);
        table[9] = Some(Piece::LIGHT_MAN);
        table[13] = Some(Piece::LIGHT_MAN);
        table[14] = Some(Piece::LIGHT_MAN);
        table[4] = Some(Piece::DARK_MAN);
        let game = CheckersGame::from_table(table);
        assert_eq!(game.capturing_pieces(), no_capturing_pieces);

        let mut table = [None; TABLE_SIZE];
        table[5] = Some(Piece::DARK_KING);
        table[6] = Some(Piece::LIGHT_MAN);
        table[9] = Some(Piece::LIGHT_MAN);
        table[13] = Some(Piece::LIGHT_MAN);
        table[14] = Some(Piece::LIGHT_MAN);
        table[4] = Some(Piece::DARK_MAN);
        let game = CheckersGame::from_table(table);
        assert_eq!(game.capturing_pieces(), no_capturing_pieces);

        let mut table = [None; TABLE_SIZE];
        table[5] = Some(Piece::DARK_KING);
        table[6] = Some(Piece::LIGHT_MAN);
        table[9] = Some(Piece::LIGHT_MAN);
        table[13] = Some(Piece::LIGHT_MAN);
        table[4] = Some(Piece::DARK_MAN);
        let game = CheckersGame::from_table(table);
        assert_eq!(game.capturing_pieces(), vec![(5, Piece::DARK_KING)]);

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
            HashSet::<_>::from_iter(vec![(9, Piece::LIGHT_MAN)].into_iter())
        );

        let mut table = [None; TABLE_SIZE];
        table[6] = Some(Piece::LIGHT_MAN);
        table[9] = Some(Piece::LIGHT_MAN);
        table[13] = Some(Piece::LIGHT_MAN);
        table[14] = Some(Piece::DARK_MAN);
        table[4] = Some(Piece::DARK_MAN);
        let game = CheckersGame::from_table(table);
        assert_eq!(
            HashSet::<_>::from_iter(game.capturing_pieces().into_iter()),
            HashSet::<_>::from_iter(vec![(9, Piece::LIGHT_MAN), (14, Piece::DARK_MAN)].into_iter())
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
            HashSet::<_>::from_iter(
                vec![
                    (9, Piece::LIGHT_MAN),
                    (5, Piece::LIGHT_MAN),
                    (8, Piece::DARK_MAN)
                ]
                .into_iter()
            )
        );
    }

    #[test]
    fn game() {
        let mut game = CheckersGame::new();
        assert_eq!(game.team_on_turn(), Team::Light);
        assert!(!game.jump(20, 16));
        assert!(!game.jump(8, 17));
        assert!(game.jump(8, 13));
        assert_eq!(game.team_on_turn(), Team::Dark);
        assert_eq!(
            game.mandatory_capturing_pieces(),
            Vec::<(usize, Piece)>::new()
        );

        assert!(!game.jump(8, 13));
        assert!(!game.jump(25, 22));
        assert!(game.jump(22, 17));
        assert_eq!(game.team_on_turn(), Team::Light);
        assert_eq!(
            game.mandatory_capturing_pieces(),
            vec![(13, Piece::LIGHT_MAN)]
        );

        assert!(!game.jump(5, 9));
        assert!(!game.jump(9, 14));
        assert!(!game.jump(14, 18));
        assert!(!game.jump(13, 16));
        assert!(game.jump(13, 22));
        assert_eq!(game.team_on_turn(), Team::Dark);
        assert_eq!(
            HashSet::<_>::from_iter(game.mandatory_capturing_pieces().into_iter()),
            HashSet::<_>::from_iter(vec![(25, Piece::DARK_MAN), (26, Piece::DARK_MAN)].into_iter())
        );
    }
}

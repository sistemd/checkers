use arrayvec::ArrayVec;

pub struct CheckersGame {
    table: Table,
    team_on_turn: Team,
}

pub type Table = [Option<Piece>; 32];

#[derive(Copy, Clone)]
pub struct Piece {
    pub team: Team,
}

#[derive(Copy, Clone)]
pub enum Team {
    Light,
    Dark,
}

impl CheckersGame {
    /// Returns those pieces which must be moved on next turn.
    /// If there are no mandatory pieces, returns an empty vector.
    pub fn mandatory_pieces(&self) -> Vec<usize> {
        vec![]
    }

    fn adjacent_positions(pos: usize) -> impl Iterator<Item = usize> {
        let row = (pos / 4) as i32;
        let offset: i32 = if row % 2 == 0 { 4 } else { 5 };

        ArrayVec::from([
            pos as i32 - offset,
            pos as i32 - offset + 1,
            pos as i32 - offset + 8,
            pos as i32 - offset + 9,
        ])
        .into_iter()
        .filter(move |&p| {
            let p_row = p / 4;
            0 <= p && p < 32 && (p_row - row).abs() == 1
        })
        .map(|p| p as usize)
    }

    /// Returns the team which is currently on turn.
    pub fn team_on_turn(&self) -> Team {
        self.team_on_turn
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
    fn adjacent_positions() {
        assert_eq!(
            CheckersGame::adjacent_positions(0).collect::<Vec<usize>>(),
            vec![4, 5]
        );
        assert_eq!(
            CheckersGame::adjacent_positions(12).collect::<Vec<usize>>(),
            vec![8, 16]
        );
        assert_eq!(
            CheckersGame::adjacent_positions(9).collect::<Vec<usize>>(),
            vec![5, 6, 13, 14]
        );
        assert_eq!(
            CheckersGame::adjacent_positions(11).collect::<Vec<usize>>(),
            vec![7, 15]
        );
        assert_eq!(
            CheckersGame::adjacent_positions(27).collect::<Vec<usize>>(),
            vec![23, 31]
        );
        assert_eq!(
            CheckersGame::adjacent_positions(28).collect::<Vec<usize>>(),
            vec![24]
        );
    }
}

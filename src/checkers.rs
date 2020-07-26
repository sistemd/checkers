pub struct CheckersGame {
    table: Table,
    active_team: Team,
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
    pub fn mandatory_pieces(&self) -> Vec<usize> {}

    /// Returns the team which is currently on turn.
    pub fn active_team(&self) -> Team {
        self.active_team
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

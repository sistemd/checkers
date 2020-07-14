use actix::prelude::*;
use std::collections::HashMap;

#[derive(Copy, Clone)]
pub struct PlayerID(i32);

struct Matchmaker {
    enqueued_player_id: Option<PlayerID>,
}

impl Matchmaker {
    fn new() -> Self {
        Self {
            enqueued_player_id: None,
        }
    }

    fn enqueue(&mut self, player_id: PlayerID) -> Option<(PlayerID, PlayerID)> {
        match self.enqueued_player_id {
            Some(p) => {
                self.enqueued_player_id = None;
                Some((p, player_id))
            }
            None => {
                self.enqueued_player_id = Some(player_id);
                None
            }
        }
    }
}

pub struct GameMaster {
    matchmaker: Matchmaker,
    games: HashMap<PlayerID, i32>,
}

impl GameMaster {
    pub fn new() -> Self {
        Self {
            matchmaker: Matchmaker::new(),
            games: HashMap::new(),
        }
    }
}

impl Actor for GameMaster {
    type Context = Context<Self>;
}

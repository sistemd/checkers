use actix::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct PlayerID(pub usize);

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GameID(pub usize);

struct Matchmaker {
    enqueued_recipient: Option<Recipient<GameFound>>,
    last_match_id: GameID,
}

impl Matchmaker {
    fn new() -> Self {
        Self {
            enqueued_recipient: None,
            last_match_id: GameID(0),
        }
    }

    fn matchup(
        &mut self,
        recipient: Recipient<GameFound>,
    ) -> Option<(Recipient<GameFound>, Recipient<GameFound>, GameID)> {
        match self.enqueued_recipient.take() {
            Some(p) => {
                self.last_match_id.0 += 1;
                Some((p, recipient, GameID(self.last_match_id.0 - 1)))
            }
            None => {
                self.enqueued_recipient = Some(recipient);
                None
            }
        }
    }
}

pub struct GameMaster {
    matchmaker: Matchmaker,
    games: HashMap<GameID, GameID>,
}

pub type GameMasterAddr = Addr<GameMaster>;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Matchup(pub Recipient<GameFound>);

#[derive(Message)]
#[rtype(result = "()")]
pub struct GameFound(pub GameID);

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

impl Handler<Matchup> for GameMaster {
    type Result = ();

    fn handle(&mut self, msg: Matchup, _: &mut Self::Context) {
        let recipient = msg.0;
        if let Some((r1, r2, game_id)) = self.matchmaker.matchup(recipient) {
            self.games.insert(game_id, game_id);
            r1.do_send(GameFound(game_id)).unwrap();
            r2.do_send(GameFound(game_id)).unwrap();
        }
    }
}

use crate::game_master;
use crate::game_master::{
    BadJump, GameFound, GameMasterAddr, GameState, GameUpdate, Matchup, PlayerID,
};
use actix::prelude::*;
use actix_web::{web, Responder};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Serialize)]
struct Registration {
    player_id: PlayerID,
}

pub struct PlayerIDCounter(AtomicUsize);

impl PlayerIDCounter {
    pub fn new() -> Self {
        PlayerIDCounter(AtomicUsize::new(0))
    }
}

/// Start a websocket connection, used for both matchmaking and carrying out active games.
async fn websocket(
    gm: web::Data<GameMasterAddr>,
    player_id_counter: web::Data<PlayerIDCounter>,
    req: web::HttpRequest,
    stream: web::Payload,
) -> impl Responder {
    ws::start(
        WsSession {
            player_id: None,
            game_master: gm.get_ref().clone(),
            player_id_counter: player_id_counter.into_inner(),
        },
        &req,
        stream,
    )
}

struct WsSession {
    player_id: Option<PlayerID>,
    game_master: GameMasterAddr,
    player_id_counter: Arc<PlayerIDCounter>,
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(msg_text)) => {
                let msg: Result<ClientMessage, _> = serde_json::from_str(&msg_text);
                match msg {
                    Ok(msg) => self.handle_valid_message(msg, ctx),
                    Err(_) => log::info!("Invalid message {}", msg_text),
                }
            }
            Ok(m) => println!("Something else {:?}", m),
            Err(e) => panic!(e),
        }
    }
}

impl WsSession {
    pub fn handle_valid_message(&mut self, msg: ClientMessage, ctx: &mut <Self as Actor>::Context) {
        match msg {
            ClientMessage::Register => self.handle_registration(ctx),
            ClientMessage::Matchup => self.handle_matchup(ctx),
            ClientMessage::Jump { from, to } => self.handle_jump(from, to),
        }
    }

    fn handle_registration(&mut self, ctx: &mut <Self as Actor>::Context) {
        if self.player_id.is_some() {
            return;
        }

        let player_id = PlayerID(self.player_id_counter.0.fetch_add(1, Ordering::AcqRel));
        let response = serde_json::to_string(&ServerMessage::Registered { player_id }).unwrap();
        ctx.text(response);
        self.player_id = Some(player_id);
    }

    fn handle_matchup(&self, ctx: &mut <Self as Actor>::Context) {
        match self.player_id {
            Some(player_id) => self.game_master.do_send(Matchup {
                game_found_recipient: ctx.address().recipient(),
                game_state_recipient: ctx.address().recipient(),
                game_update_recipient: ctx.address().recipient(),
                bad_jump_recipient: ctx.address().recipient(),
                player_id,
            }),
            None => unimplemented!(),
        }
    }

    fn handle_jump(&self, from: usize, to: usize) {
        match self.player_id {
            Some(player_id) => self.game_master.do_send(game_master::Jump {
                player_id,
                from,
                to,
            }),
            None => unimplemented!(),
        }
    }
}

impl Handler<GameFound> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: GameFound, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(serde_json::to_string(&ServerMessage::GameFound(msg)).unwrap());
    }
}

impl Handler<BadJump> for WsSession {
    type Result = ();

    fn handle(&mut self, _: BadJump, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(serde_json::to_string(&ServerMessage::BadJump).unwrap())
    }
}

impl Handler<GameState> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: GameState, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(serde_json::to_string(&ServerMessage::GameState(msg)).unwrap())
    }
}

impl Handler<GameUpdate> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: GameUpdate, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(serde_json::to_string(&ServerMessage::GameUpdate(msg)).unwrap())
    }
}

#[derive(Deserialize)]
enum ClientMessage {
    Register,
    Matchup,
    Jump { from: usize, to: usize },
}

#[derive(Serialize)]
enum ServerMessage {
    Registered { player_id: PlayerID },
    GameFound(GameFound),
    GameState(GameState),
    GameUpdate(GameUpdate),
    BadJump,
}

pub fn config_with(
    gm: GameMasterAddr,
    player_id_counter: web::Data<PlayerIDCounter>,
) -> impl FnOnce(&mut web::ServiceConfig) -> () {
    move |cfg: &mut web::ServiceConfig| {
        cfg.service(
            web::resource("/ws")
                .app_data(player_id_counter.clone())
                .data(gm.clone())
                .route(web::get().to(websocket)),
        );
    }
}

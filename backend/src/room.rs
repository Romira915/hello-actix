use std::collections::HashMap;

use actix::{Actor, Context, Handler, Recipient, Supervised};
use log::{debug, info};

use crate::message::{ChatMessage, SendMessage};

pub struct User {
    score: usize,
    client: Recipient<ChatMessage>,
}

enum QuizLifecycle {
    Ready,
    Wait,
    Starting,
    Started,
    Stopping,
    Stopped,
}

impl Default for QuizLifecycle {
    fn default() -> Self {
        Self::Ready
    }
}

#[derive(Default)]
pub struct QuizRoom {
    room_name: String,
    pub users: HashMap<usize, User>,
    state: QuizLifecycle,
}

impl QuizRoom {
    pub(crate) fn new(room_name: String) -> Self {
        Self {
            room_name,
            ..Default::default()
        }
    }
}

impl Actor for QuizRoom {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("Quiz room {} started", self.room_name);
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        info!("Quiz room {} stopped", self.room_name);
    }
}

impl Handler<SendMessage> for QuizRoom {
    type Result = ();

    fn handle(&mut self, msg: SendMessage, _ctx: &mut Self::Context) {
        let SendMessage(room_name, id, msg) = msg;
        debug!("room_name {}", room_name);
    }
}

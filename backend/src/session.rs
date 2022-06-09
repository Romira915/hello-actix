use actix::{
    fut, Actor, ActorContext, ActorFuture, AsyncContext, Context, ContextFutureSpawner, Handler,
    StreamHandler, SystemService, WrapFuture,
};
use actix_broker::BrokerIssue;
use actix_daemon_utils::{
    delayer::{Delayer, Task, Timing},
    graceful_stop::GracefulStop,
};
use actix_web_actors::ws;
use log::{debug, info};
use tokio::time::{sleep, Duration};

use crate::{
    message::{ChatMessage, JoinRoom, LeaveRoom, ListRooms, SendMessage},
    server::WsQuizServer,
};

#[derive(Default)]
pub struct DelayActor {
    msg: String,
    ms: u64,
    count: usize,
}

impl Actor for DelayActor {
    type Context = Context<Self>;
}

impl Handler<Task> for DelayActor {
    type Result = ();

    fn handle(&mut self, msg: Task, ctx: &mut Self::Context) -> Self::Result {
        if self.count == 0 {
            msg.0.do_send(Timing::Later(Duration::from_millis(self.ms)));
            self.count += 1;
            return;
        }

        let msg = SendMessage("Main".to_string(), 0, "delay send".to_string());

        // issue_async comes from having the `BrokerIssue` trait in scope.
        self.issue_system_async(msg);
        debug!("send! delay! {}", self.count);
        self.count += 1;
        ctx.stop();
    }
}

#[derive(Default)]
pub struct WsSession {
    id: usize,
    room: String,
    name: Option<String>,
}

impl WsSession {
    pub fn join_room(&mut self, room_name: &str, ctx: &mut ws::WebsocketContext<Self>) {
        let room_name = room_name.to_owned();

        // First send a leave message for the current room
        let leave_msg = LeaveRoom(self.room.clone(), self.id);

        // issue_sync comes from having the `BrokerIssue` trait in scope.
        self.issue_system_sync(leave_msg, ctx);

        // Then send a join message for the new room
        let join_msg = JoinRoom(
            room_name.to_owned(),
            self.name.clone(),
            ctx.address().recipient(),
        );

        WsQuizServer::from_registry()
            .send(join_msg)
            .into_actor(self)
            .then(|id, act, _ctx| {
                if let Ok(id) = id {
                    act.id = id;
                    act.room = room_name;
                }

                fut::ready(())
            })
            .wait(ctx);
    }

    pub fn list_rooms(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        WsQuizServer::from_registry()
            .send(ListRooms)
            .into_actor(self)
            .then(|res, _, ctx| {
                if let Ok(rooms) = res {
                    for room in rooms {
                        ctx.text(room);
                    }
                }

                fut::ready(())
            })
            .wait(ctx);
    }

    pub fn send_msg(&self, msg: &str) {
        let content = format!(
            "{}: {}",
            self.name.clone().unwrap_or_else(|| "anon".to_string()),
            msg
        );

        let msg = SendMessage(self.room.clone(), self.id, content);

        // issue_async comes from having the `BrokerIssue` trait in scope.
        self.issue_system_async(msg);
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("WsSession connected");
        self.join_room("Main", ctx);
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        info!(
            "WsSession closed for {}({}) in room {}",
            self.name.clone().unwrap_or_else(|| "anon".to_string()),
            self.id,
            self.room
        );
    }
}

impl Handler<ChatMessage> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: ChatMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        debug!("WEBSOCKET MESSAGE: {:?}", msg);

        match msg {
            ws::Message::Text(text) => {
                let msg = text.trim();

                if msg.starts_with('/') {
                    let mut command = msg.splitn(2, ' ');

                    match command.next() {
                        Some("/list") => self.list_rooms(ctx),

                        Some("/join") => {
                            if let Some(room_name) = command.next() {
                                self.join_room(room_name, ctx);
                            } else {
                                ctx.text("!!! room name is required");
                            }
                        }

                        Some("/name") => {
                            if let Some(name) = command.next() {
                                self.name = Some(name.to_owned());
                                ctx.text(format!("name changed to: {}", name));
                            } else {
                                ctx.text("!!! name is required");
                            }
                        }

                        Some("/start") => {
                            let graceful_stop = GracefulStop::new();
                            let actor = DelayActor {
                                msg: "hello".to_string(),
                                ms: 3000,
                                count: 0,
                            }
                            .start();
                            let delayer = Delayer::new(
                                actor.recipient(),
                                graceful_stop.clone_system_terminator(),
                                Duration::from_secs(10),
                            )
                            .start();

                            graceful_stop.subscribe(delayer.recipient()).start();
                        }

                        _ => ctx.text(format!("!!! unknown command: {:?}", msg)),
                    }

                    return;
                }
                self.send_msg(msg);
            }
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => {}
        }
    }
}

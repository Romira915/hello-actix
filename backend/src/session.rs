use actix::Actor;
use actix_web_actors::ws;
use log::info;

#[derive(Default)]
pub struct WsSession {
    id: usize,
    room: String,
    name: Option<String>,
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {}

    fn stopped(&mut self, ctx: &mut Self::Context) {
        info!(
            "WsSession closed for {}({}) in room {}",
            self.name.clone().unwrap_or_else(|| "anon".to_string()),
            self.id,
            self.room
        );
    }
}

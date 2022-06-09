extern crate hello_actix;

use std::{
    fs::{self, File},
    path::Path,
};

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{
    get, middleware::Logger, post, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    Responder,
};
use actix_web_actors::ws;
use chrono::{FixedOffset, Utc};
use hello_actix::session::WsSession;
use log::{info, LevelFilter};
use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, SharedLogger, TermLogger, TerminalMode, WriteLogger,
};

async fn ws_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(WsSession::default(), &req, stream)
}

fn init_logger<P>(log_path: Option<P>)
where
    P: AsRef<Path>,
{
    let jst_now = {
        let jst = Utc::now();
        jst.with_timezone(&FixedOffset::east(9 * 3600))
    };
    let mut logger: Vec<Box<dyn SharedLogger>> = vec![
        #[cfg(not(feature = "termcolor"))]
        TermLogger::new(
            LevelFilter::Debug,
            ConfigBuilder::new().set_time_to_local(true).build(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
    ];
    if let Some(log_path) = log_path {
        fs::create_dir_all(&log_path).unwrap();
        logger.push(WriteLogger::new(
            LevelFilter::Warn,
            ConfigBuilder::new().set_time_to_local(true).build(),
            File::create(log_path.as_ref().join(format!("{}.log", jst_now))).unwrap(),
        ));
    }
    CombinedLogger::init(logger).unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger(Some(&Path::new("./log")));

    info!("Starting http server");

    HttpServer::new(|| {
        App::new()
            // .wrap(
            //     Cors::default()
            //         .allowed_origin("All")
            //         .send_wildcard()
            //         .max_age(3600),
            // )
            .wrap(Logger::default())
            .service(web::resource("/ws/").to(ws_route))
            .service(Files::new("/", "./static/").index_file("index.html"))
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}

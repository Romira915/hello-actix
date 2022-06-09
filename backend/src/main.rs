<<<<<<< HEAD
extern crate hello_actix;

=======
>>>>>>> origin/main
use std::{
    fs::{self, File},
    path::Path,
};

use actix_cors::Cors;
use actix_files::Files;
<<<<<<< HEAD
use actix_web::{
    get, middleware::Logger, post, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    Responder,
};
use actix_web_actors::ws;
use chrono::{FixedOffset, Utc};
use hello_actix::session::WsSession;
use log::{info, LevelFilter};
=======
use actix_web::{middleware::Logger, web, App, HttpRequest, HttpServer};
use build_timestamp::build_time;
use chrono::{FixedOffset, Utc};
use log::LevelFilter;
>>>>>>> origin/main
use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, SharedLogger, TermLogger, TerminalMode, WriteLogger,
};

<<<<<<< HEAD
async fn ws_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(WsSession::default(), &req, stream)
}

fn init_logger<P>(log_path: Option<P>)
where
    P: AsRef<Path>,
{
=======
mod quiz;
mod sample_ws;

build_time!("%A %Y-%m-%d / %H:%M:%S");

async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    println!("built on: {}", BUILD_TIME);
    BUILD_TIME
}

fn init_logger(log_path: Option<&str>) {
>>>>>>> origin/main
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
<<<<<<< HEAD
=======
        let log_path = Path::new(log_path);
>>>>>>> origin/main
        fs::create_dir_all(&log_path).unwrap();
        logger.push(WriteLogger::new(
            LevelFilter::Warn,
            ConfigBuilder::new().set_time_to_local(true).build(),
<<<<<<< HEAD
            File::create(log_path.as_ref().join(format!("{}.log", jst_now))).unwrap(),
=======
            File::create(log_path.join(format!("{}.log", jst_now))).unwrap(),
>>>>>>> origin/main
        ));
    }
    CombinedLogger::init(logger).unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
<<<<<<< HEAD
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
=======
    // init_logger(Some(&Path::new("./log")));
    init_logger(None);

    std::env::set_var("RUST_LOG", "actix_web=info");

    HttpServer::new(|| {
        App::new()
            .wrap(if cfg!(debug_assertions) {
                Cors::permissive()
            } else {
                Cors::default()
                    .allowed_origin("All")
                    .send_wildcard()
                    .max_age(3600)
            })
            .wrap(Logger::default())
            .service(web::resource("/").to(index))
            .service(quiz::quiz)
            .service(Files::new("/static", "./backend/static/").index_file("index.html"))
>>>>>>> origin/main
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}

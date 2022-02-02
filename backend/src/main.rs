use std::{
    fs::{self, File},
    path::Path,
};

use actix_cors::Cors;
use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::{FixedOffset, Utc};
use log::LevelFilter;
use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, SharedLogger, TermLogger, TerminalMode, WriteLogger,
};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello actix from Azure Web Apps!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
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
            LevelFilter::Info,
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

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("All")
                    .send_wildcard()
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}

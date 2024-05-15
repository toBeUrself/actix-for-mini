#[macro_use]
extern crate dotenv_codegen;
extern crate derive_more;

mod env;
mod models;
mod mysql;
mod traits;

use crate::mysql::fetch_shops;
use ::mysql::Pool;
use actix_cors::Cors;
use actix_web::{get, http::StatusCode, post, web, App, HttpResponse, HttpServer, Responder};
use env::{MYSQL_HOST, MYSQL_PORT, MYSQL_PWD, MYSQL_USER};
use mysql::get_conn_builder;
use std::io::Result;

#[get("/")]
pub(crate) async fn hello(data: web::Data<Pool>) -> actix_web::Result<impl Responder> {
    let data = web::block(move || fetch_shops(&data)).await??;

    Ok(web::Json(data))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("initializing database connection");

    let opts = get_conn_builder(
        MYSQL_USER,
        MYSQL_PWD,
        MYSQL_HOST,
        MYSQL_PORT.parse().unwrap(),
        "mysql",
    );

    let pool = Pool::new(opts).unwrap();

    let share_data = web::Data::new(pool);

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(share_data.clone())
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

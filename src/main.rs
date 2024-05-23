#[macro_use]
extern crate dotenv_codegen;
extern crate derive_more;

mod apperror;
mod env;
mod models;
mod mysql;
mod routes;
mod traits;

use crate::{
    mysql::common::get_conn_builder, routes::{
        glasses::{del_glasse, get_glasse_list, post_glasse, put_glasse, upload_file},
        shop::shop_list,
    }
};
use ::mysql::Pool;
use actix_cors::Cors;
use actix_web::{error, get, middleware::Logger, post, web, App, HttpMessage, HttpResponse, HttpServer, Responder};
use env::{MYSQL_HOST, MYSQL_PORT, MYSQL_PWD, MYSQL_USER};
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};
use std::io::Result;
use crate::routes::shop::__path_shop_list;

use  models::{common::ApiResult, shop::Shop};

#[get("/")]
pub(crate) async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
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

    log::info!("initializing database connection => MYSQL_USER: {}, MYSQL_PWD: {}, MYSQL_HOST: {}, MYSQL_PORT: {}", MYSQL_USER, MYSQL_PWD, MYSQL_HOST, MYSQL_PORT);

    #[derive(OpenApi)]
    #[openapi(
        paths(shop_list),
        components(schemas(ApiResult<Vec<Shop>>)),
        tags(
            (name = "微信小程序服务", description = "with rust")
        )
    )]
    struct ApiDoc;

    let openapi = ApiDoc::openapi();

    let opts = get_conn_builder(
        MYSQL_USER,
        MYSQL_PWD,
        MYSQL_HOST,
        MYSQL_PORT.parse().unwrap(),
        "mysql",
    );

    let pool = Pool::new(opts).unwrap();

    let share_data = web::Data::new(pool);

    log::info!("starting HTTP server at http://localhost:3000");

    HttpServer::new(move || {
        let cors = Cors::permissive();
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                log::info!("error_handler {:?}", err);
                println!(
                    "error_handler {:?} {:?} {:?}",
                    _req.method(),
                    _req.app_config(),
                    _req
                );

                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(share_data.clone())
            .app_data(json_config)
            .service(hello)
            .service(
                web::scope("/rust")
                    .service(shop_list)
                    .service(get_glasse_list)
                    .service(post_glasse)
                    .service(upload_file)
                    .service(put_glasse)
                    .service(del_glasse)
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}

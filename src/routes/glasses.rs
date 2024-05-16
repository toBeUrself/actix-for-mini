use actix_web::{get, post, web, Responder};
use mysql::Pool;

use crate::{
    models::glasses::{Glasse, GlasseInsert, GlassesListForm},
    mysql::glasses::{fetch_glasses, insert_glasse},
};

#[get("/glasse-list")]
pub(crate) async fn get_glasse_list(
    params: web::Query<GlassesListForm>,
    data: web::Data<Pool>,
) -> actix_web::Result<impl Responder> {
    log::info!("/glasse-list => {:?}", params.0);

    let data = web::block(move || fetch_glasses(&data, params.page, params.size)).await??;

    Ok(web::Json(data))
}

#[post("/insert-glasse")]
pub(crate) async fn post_glasse(
    glasse: web::Json<GlasseInsert>,
    data: web::Data<Pool>,
) -> actix_web::Result<impl Responder> {
    log::info!("/insert-glasse => {:?}", glasse.0);

    let data = web::block(move || insert_glasse(&data, glasse.0)).await??;

    Ok(web::Json(data))
}

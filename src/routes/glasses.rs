use actix_multipart::form::MultipartForm;
use actix_web::{get, post, put, web, Responder};
use mysql::Pool;

use crate::{
    models::glasses::{Glasse, GlasseInsert, GlassesListForm, UploadFormData},
    mysql::glasses::{fetch_glasses, insert_glasse, save_files, update_glasse},
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

#[put("update-glasse")]
pub(crate) async fn put_glasse(
    glasse: web::Json<Glasse>,
    data: web::Data<Pool>,
) -> actix_web::Result<impl Responder> {
    log::info!("/update-glasse => {:?}", glasse.0);

    let data = web::block(move || update_glasse(&data, glasse.0)).await??;

    Ok(web::Json(data))
}

#[post("/upload-file")]
pub(crate) async fn upload_file(
    MultipartForm(form): MultipartForm<UploadFormData>,
    data: web::Data<Pool>,
) -> actix_web::Result<impl Responder> {
    log::info!("/upload-file => {:?}", form);

    let data = web::block(move || save_files(&data, form)).await??;

    Ok(web::Json(data))
}

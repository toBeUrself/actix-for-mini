use actix_multipart::form::MultipartForm;
use actix_web::{post, web, Responder};
use crate::{models::file::UploadImageFormData, mysql::file::save_images};

#[post("/upload-image")]
pub(crate) async fn upload_image(
  MultipartForm(form): MultipartForm<UploadImageFormData>,
) -> actix_web::Result<impl Responder>{
  log::info!("/upload-image => {:?}", form);

  let data = web::block(move || save_images(form)).await??;

    Ok(web::Json(data))
}
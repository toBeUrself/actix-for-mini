use std::fs::File;

use actix_multipart::{form::MultipartForm, Multipart};
use actix_web::{http::StatusCode, post, web, Responder};
use crate::{env::SAVE_DIR, models::{common::ApiResult, file::UploadImageFormData}, mysql::file::save_images};

#[post("/upload-image")]
pub(crate) async fn upload_image(
  MultipartForm(form): MultipartForm<UploadImageFormData>,
) -> actix_web::Result<impl Responder>{
  log::info!("/upload-image => {:?}", form);

  let data = web::block(move || save_images(form)).await??;

    Ok(web::Json(data))
}

// #[post("/upload-mini-image")]
// pub(crate) async fn upload_mini_image(
//   mut payload: Multipart,
// ) -> actix_web::Result<impl Responder> {
//   while let Ok(Some(mut field)) = payload.try_next().await {
// 		let content_disposition = field.content_disposition();
// 		let file_name = content_disposition.get_filename().unwrap();

//     let path = format!("{}/{}", SAVE_DIR, file_name.unwrap());
// 		// 获取临时目录路径
// 		// let mut file_path = env::temp_dir();
// 		// file_path.push(file_name);
// 		// println!("---full file_path:{}, file_name:{}", file_path.display(), file_name);

// 		let mut file = File::create(path)?;
// 		while let Some(chunk) = field.next().await {
// 			let data = chunk?;
// 			file.write_all(&data)?;
// 		}
// 	}

//   Ok(web::Json(ApiResult {
//     code: StatusCode::OK.into(),
//     data: "success".to_string(),
//     msg: Some("success".into()),
//   }))
// }
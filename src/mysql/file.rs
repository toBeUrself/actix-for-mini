use actix_web::http::StatusCode;

use crate::{apperror::AppError, env::SAVE_DIR, models::{common::ApiResult, file::UploadImageFormData}};

pub fn save_images(form: UploadImageFormData) -> Result<ApiResult<String>, AppError> {
    for fs in form.files {
        let path = format!("{}/{}", SAVE_DIR, fs.file_name.unwrap());

        log::info!("saving to {path}");

        fs.file.persist(path).unwrap();
    }

    Ok(ApiResult {
        code: StatusCode::OK.into(),
        data: "success".to_string(),
        msg: Some("success".into()),
    })
}
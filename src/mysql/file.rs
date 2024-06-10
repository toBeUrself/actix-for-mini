use std::{fs::File, io::Write};

use actix_web::http::StatusCode;

use crate::{apperror::AppError, env::SAVE_DIR, models::{common::ApiResult, file::UploadImageFormData}};

const MAX_FILE_SIZE: usize = 1024 * 1024 * 10; // 10 MB
const MAX_FILE_COUNT: i32 = 1;

pub fn save_images(form: UploadImageFormData) -> Result<ApiResult<String>, AppError> {
    match form.file.size {
        0 => Ok(ApiResult {
            code: StatusCode::BAD_REQUEST.into(),
            data: "没有上传任何文件".to_string(),
            msg: Some("failed".into()),
        }),
        length if length > MAX_FILE_SIZE => Ok(ApiResult {
            code: StatusCode::BAD_REQUEST.into(),
            data: format!("文件大小不能超过{}字节，10m", MAX_FILE_SIZE),
            msg: Some("failed".into()),
        }),
        _ => {
            let path = format!("{}/{}", SAVE_DIR, form.file.file_name.unwrap());
            log::info!("saving to {path}");

            form.file.file.persist(path).unwrap();


            Ok(ApiResult {
                code: StatusCode::OK.into(),
                data: "success".to_string(),
                msg: Some("success".into()),
            })
        },
    }
}
use actix_multipart::form::{tempfile::TempFile, MultipartForm};

#[derive(Debug, MultipartForm)]
pub struct UploadImageFormData {
    #[multipart(rename = "file")]
    pub files: Vec<TempFile>,
}
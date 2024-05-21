use crate::{
    apperror::AppError,
    models::{
        common::ApiResult,
        glasses::{Glasse, GlasseInsert, UploadFormData},
    },
};
use actix_web::http::StatusCode;
use mysql::{params, prelude::*, Pool, PooledConn};

pub fn insert_glasse(pool: &Pool, glasse: GlasseInsert) -> Result<ApiResult<u64>, AppError> {
    let mut conn = pool.get_conn()?;

    let res = insert_glasse_sql(&mut conn, glasse)?;

    Ok(ApiResult {
        code: StatusCode::OK.into(),
        data: res,
        msg: Some("success".into()),
    })
}

pub fn update_glasse(
    pool: &Pool,
    glasse: Glasse,
) -> Result<ApiResult<u64>, AppError> {
    let mut conn = pool.get_conn()?;

    let res = update_glasse_sql(&mut conn, glasse)?;

    Ok(ApiResult {
        code: StatusCode::OK.into(),
        data: res,
        msg: Some("success".into()),
    })
}

pub fn insert_glasse_sql(conn: &mut PooledConn, glasse: GlasseInsert) -> mysql::error::Result<u64> {
    conn.exec_drop(r"
        INSERT INTO wechat_mini_app.glasses (name, email, `type`, style, description, telephone, creator)
        VALUES (:name, :email, :type, :style, :description, :telephone, :creator)
    ",
    params! {
        "name" => glasse.name,
        "email" => glasse.email,
        "type" => glasse.r#type,
        "style" => glasse.style,
        "description" => glasse.description,
        "telephone" => glasse.telephone,
        "creator" => glasse.creator,
        },
    ).map(|_| conn.last_insert_id())
}

pub fn update_glasse_sql(
    conn: &mut PooledConn,
    glasse: Glasse,
) -> mysql::error::Result<u64> {
    conn.exec_drop(r"
        UPDATE wechat_mini_app.glasses SET name = :name, type = :type, style = :style, description = :description
            WHERE id = :id
    ",
    params! {
        "id" => glasse.id,
        "name" => glasse.name,
        "type" => glasse.r#type,
        "style" => glasse.style,
        "description" => glasse.description,
    },
    ).map(|_| {
        glasse.id
    })
}

pub fn fetch_glasses(
    pool: &Pool,
    page: u32,
    size: u32,
) -> Result<ApiResult<Vec<Glasse>>, AppError> {
    let mut conn = pool.get_conn()?;

    let shops = query_glasses(&mut conn, page, size)?;

    Ok(ApiResult {
        code: StatusCode::OK.into(),
        data: shops,
        msg: Some("success".into()),
    })
}

pub fn query_glasses(
    conn: &mut PooledConn,
    page: u32,
    size: u32,
) -> mysql::error::Result<Vec<Glasse>> {
    conn.exec_map(
        r"
        SELECT id, name, email, type, style, description, img_url, telephone, create_time, update_time, creator
        FROM wechat_mini_app.glasses
        ORDER BY id ASC
        limit :page,:size
    ",
        params! {
            "page" => (page - 1) * size,
            "size" => size,
        },
        |(id,
            name,
            email,
            r#type,
            style,
            description,
            img_url,
            telephone,
            create_time,
            update_time,
            creator)| Glasse {
            id,
            name,
            email,
            r#type,
            style,
            description,
            img_url,
            telephone,
            create_time,
            update_time,
            creator,
        },
    )
}

pub fn save_files(conn: &Pool, form: UploadFormData) -> Result<ApiResult<String>, AppError> {
    for fs in form.files {
        let path = format!("./tmp/{}", fs.file_name.unwrap());

        log::info!("saving to {path}");

        fs.file.persist(path).unwrap();
    }

    Ok(ApiResult {
        code: StatusCode::OK.into(),
        data: form.name.to_string(),
        msg: Some("success".into()),
    })
}

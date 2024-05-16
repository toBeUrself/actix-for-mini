use crate::models::{
    common::ApiResult,
    glasses::{Glasse, GlasseInsert},
};
use actix_web::http::StatusCode;
use derive_more::{Display, Error, From};
use mysql::{params, prelude::*, Pool, PooledConn};

#[derive(Debug, Display, Error, From)]
pub enum PersistenceError {
    EmptyPage,
    EmptySize,
    MysqlError(mysql::Error),
    Unknown,
}

impl actix_web::ResponseError for PersistenceError {
    fn status_code(&self) -> StatusCode {
        match self {
            PersistenceError::EmptyPage | PersistenceError::EmptySize => StatusCode::BAD_REQUEST,

            PersistenceError::MysqlError(_) | PersistenceError::Unknown => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

pub fn insert_glasse(
    pool: &Pool,
    glasse: GlasseInsert,
) -> Result<ApiResult<u64>, PersistenceError> {
    let mut conn = pool.get_conn()?;

    let res = insert_glasse_sql(&mut conn, glasse)?;

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
    ", params! {
        "name" => glasse.name,
        "email" => glasse.email,
        "type" => glasse.r#type,
        "style" => glasse.style,
        "description" => glasse.descriptoin,
        "telephone" => glasse.telephone,
        "creator" => glasse.creator,
    },
).map(|_| conn.last_insert_id())
}

pub fn fetch_glasses(
    pool: &Pool,
    page: u32,
    size: u32,
) -> Result<ApiResult<Vec<Glasse>>, PersistenceError> {
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
            descriptoin,
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
            descriptoin,
            img_url,
            telephone,
            create_time,
            update_time,
            creator,
        },
    )
}

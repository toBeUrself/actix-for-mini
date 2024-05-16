use crate::models::{common::ApiResult, shop::Shop};
use actix_web::http::StatusCode;
use derive_more::{Display, Error, From};
use mysql::{params, prelude::*, Pool, PooledConn};

#[derive(Debug, Display, Error, From)]
pub enum PersistenceError {
    EmptyBankName,
    EmptyCountry,
    EmptyBranch,
    EmptyLocation,
    EmptyTellerName,
    EmptyCustomerName,
    MysqlError(mysql::Error),
    Unknown,
}

impl actix_web::ResponseError for PersistenceError {
    fn status_code(&self) -> StatusCode {
        match self {
            PersistenceError::EmptyBankName
            | PersistenceError::EmptyCountry
            | PersistenceError::EmptyBranch
            | PersistenceError::EmptyLocation
            | PersistenceError::EmptyTellerName
            | PersistenceError::EmptyCustomerName => StatusCode::BAD_REQUEST,

            PersistenceError::MysqlError(_) | PersistenceError::Unknown => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

pub fn fetch_shops(
    pool: &Pool,
    page: u32,
    size: u32,
) -> Result<ApiResult<Vec<Shop>>, PersistenceError> {
    let mut conn = pool.get_conn()?;

    let shops = query_shops(&mut conn, page, size)?;

    Ok(ApiResult {
        code: StatusCode::OK.into(),
        data: shops,
        msg: Some("success".into()),
    })
}

pub fn query_shops(conn: &mut PooledConn, page: u32, size: u32) -> mysql::error::Result<Vec<Shop>> {
    conn.exec_map(
        r"
      SELECT id, name, email, telephone, creator, status, active, create_time, update_time
      FROM wechat_mini_app.shops
      ORDER BY id ASC
      limit :page,:size
    ",
        params! {
            "page" => (page - 1) * size,
            "size" => size,
        },
        |(id, name, email, telephone, creator, status, active, create_time, update_time)| {
            log::info!("{} {} {:?}", id, name, create_time);

            Shop {
                id,
                name,
                email,
                telephone,
                creator,
                status,
                active,
                create_time,
                update_time,
            }
        },
    )
}

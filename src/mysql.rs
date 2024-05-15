use actix_web::http::StatusCode;
use derive_more::{Display, Error, From};
use mysql::{prelude::*, Pool, PooledConn};

use crate::models::{ApiResult, Shop};

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

pub fn get_conn_builder(
    user: &str,
    pwd: &str,
    host: &str,
    port: u16,
    name: &str,
) -> mysql::OptsBuilder {
    mysql::OptsBuilder::new()
        .ip_or_hostname(Some(host))
        .tcp_port(port)
        .db_name(Some(name))
        .user(Some(user))
        .pass(Some(pwd))
}

pub fn fetch_shops(pool: &Pool) -> Result<ApiResult<Vec<Shop>>, PersistenceError> {
    let mut conn = pool.get_conn()?;

    let shops = query_shops(&mut conn)?;

    Ok(ApiResult {
        code: StatusCode::OK.into(),
        data: shops,
        msg: Some("success".into()),
    })
}

pub fn query_shops(conn: &mut PooledConn) -> mysql::error::Result<Vec<Shop>> {
    conn.query_map(
        r"
      SELECT id, name, email, telephone, creator, status, active, create_time, update_time
      FROM wechat_mini_app.shops
      ORDER BY id ASC
    ",
        |(id, name, email, telephone, creator, status, active, create_time, update_time)| Shop {
            id,
            name,
            email,
            telephone,
            creator,
            status,
            active,
            create_time,
            update_time,
        },
    )
}

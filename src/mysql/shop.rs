use crate::{apperror::AppError, models::{common::ApiResult, shop::Shop}};
use actix_web::http::StatusCode;
use mysql::{params, prelude::*, Pool, PooledConn};

pub fn fetch_shops(
    pool: &Pool,
    page: u32,
    size: u32,
) -> Result<ApiResult<Vec<Shop>>, AppError> {
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

use actix_web::{get, web, Responder};
use mysql::Pool;

use crate::{models::shop::ShopListForm, mysql::shop::fetch_shops};

#[get("/shop-list")]
pub(crate) async fn get_shop_list(
    params: web::Query<ShopListForm>,
    data: web::Data<Pool>,
) -> actix_web::Result<impl Responder> {
    log::info!("/shop-list => {:?}", params.0);

    let data = web::block(move || fetch_shops(&data, params.page, params.size)).await??;

    Ok(web::Json(data))
}

use actix_web::{get, web, Responder};
use mysql::Pool;

use crate::{models::shop::ShopListForm, mysql::shop::fetch_shops};

#[utoipa::path(
    tag = "获取商家列表",
    context_path = "/rust",
    params(
        ShopListForm
    ),
    responses(
        (status = 200, description = "查询商家列表", body = ApiResultWithShop) 
    ),
    security(
        ("api_key" = [])
    ),
)]
#[get("/shop-list")]
pub(crate) async fn shop_list(
    params: web::Query<ShopListForm>,
    data: web::Data<Pool>,
) -> actix_web::Result<impl Responder> {
    let data = web::block(move || fetch_shops(&data, params.page, params.size)).await??;

    Ok(web::Json(data))
}

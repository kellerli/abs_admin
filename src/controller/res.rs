use actix_web::{HttpResponse, Responder, web};
use rbatis::plugin::page::{Page};
use crate::dao::RB;
use crate::domain::BizRes;
use crate::dto::ResPageDTO;
use crate::service::RES_SERVICE;

/// 资源分页(json请求)
pub async fn res_page(page: web::Json<ResPageDTO>) -> impl Responder {
    let data = RES_SERVICE.page(&page.0).await;
    if data.is_err() {
        return HttpResponse::Ok().body(data.err().unwrap().to_string());
    }
    HttpResponse::Ok().content_type("json").body(data.unwrap().to_string())
}
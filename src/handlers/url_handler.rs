// url_handler 模块
use actix_web::{get, post, web, HttpResponse};
use crate::models::url::{CreateUrlRequest, CreateUrlResponse, Url};
use crate::models::response::ApiResponse;
use crate::utils::generate_random_key;
use crate::config::Config;

#[post("/api/urls")]
pub async fn generate_url(request: web::Json<CreateUrlRequest>, config: web::Data<Config>) -> HttpResponse {
    if request.original_url.is_empty() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(400, String::from("原始链接不能为空")));
    }
    if request.original_url.len() > 2048 {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(400, String::from("原始链接长度不能超过 2048")));
    }
    if !request.original_url.starts_with("http://") && !request.original_url.starts_with("https://") {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(400, String::from("原始链接必须以 http:// 或 https:// 开头")));
    }
    
    let short_name = generate_random_key();
    
    match Url::create(request.original_url.clone(), short_name.clone()) {
        Ok(url) => {
            let response = CreateUrlResponse {
                short_name: url.short_name.clone(),
                short_url: format!("{}/{}", config.server.base_url, url.short_name.clone())
            };
            HttpResponse::Ok().json(ApiResponse::success(response))
        },
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(500, String::from("保存短链接失败")))
    }
}

#[get("/api/urls")]
pub async fn get_urls() -> HttpResponse {
    Url::find_all().map(|urls| {
        HttpResponse::Ok().json(ApiResponse::success(urls))
    }).unwrap_or_else(|_| {
        HttpResponse::InternalServerError().json(ApiResponse::<()>::error(500, String::from("数据库查询失败")))
    })
}

#[get("/{short_name}")]
pub async fn to_short_url(short_name: web::Path<String>) -> HttpResponse {
    match Url::find_by_short_name(&short_name) {
        Ok(result) => match result {
            Some(original_url) => {
                HttpResponse::Found().header("Location", original_url).finish()
            },
            None => {
                HttpResponse::NotFound().json(ApiResponse::<()>::error(404, String::from("短链接不存在")))
            }
        },
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(500, String::from("数据库查询失败")))
    }
}
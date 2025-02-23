use actix_web::{get, web, App, HttpResponse, HttpServer};
use mysql::{prelude::Queryable, Pool, OptsBuilder};
use std::env;

#[get("/{short_name}")]
async fn to_short_url(short_name: web::Path<String>) -> HttpResponse {
    let opts = OptsBuilder::new()
    .ip_or_hostname(Some(env::var("DB_HOST").expect("DB_HOST must be set")))
    .user(Some(env::var("DB_USERNAME").expect("DB_USERNAME must be set")))
    .pass(Some(env::var("DB_PASSWORD").expect("DB_PASSWORD must be set")))
    .db_name(Some(env::var("DB_DATABASE").expect("DB_DATABASE must be set")))
    .tcp_port(
        env::var("DB_PORT")
            .expect("DB_PORT must be set")
            .parse::<u16>() // 将字符串解析为 u16
            .expect("Failed to parse DB_PORT as a number"), // 如果解析失败，抛出错误
    );
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();
    // 查询数据库
    let query = format!("SELECT original_url FROM urls WHERE short_name = '{}'", short_name);
    let result: Option<String> = conn.query_first(query).unwrap();

    match result {
        Some(original_url) => {
            HttpResponse::Found().header("Location", original_url).finish()
        },
        None => {
            // 如果未找到，返回 404 Not Found
            HttpResponse::NotFound().body("Short URL not found")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 从环境变量加载配置
    dotenv::dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .service(to_short_url)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

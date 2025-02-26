mod config;
mod db;
mod handlers;
mod models;
mod utils;
mod logging;

use actix_files as fs;
use actix_web::{App, HttpServer, web};
use crate::config::Config;
use crate::db::init_pool;
use crate::handlers::url_handler::{generate_url, to_short_url, get_urls};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 从环境变量加载配置
    dotenv::dotenv().ok();

    // 初始化配置
    let config = Config::from_env();
    let config = web::Data::new(config); // 这里是 Data<Config> 的新实例

    // 初始化数据库连接池
    init_pool().expect("Failed to initialize database pool");

    println!("Server running at: {}:{}", config.server.host, config.server.port);

    // 启动HTTP服务器
    let bind_config = config.clone(); // 为 bind 操作创建一个新的引用
    HttpServer::new(move || {
        App::new()
            .app_data(config.clone()) // 使用 clone() 而不是直接使用 config
            .service(generate_url)
            .service(to_short_url)
            .service(get_urls)
            .service(fs::Files::new("/", "static").index_file("index.html"))
    })
    .bind(format!("{0}:{1}", bind_config.server.host, bind_config.server.port))? // 修正格式
    .run()
    .await
}

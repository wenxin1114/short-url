use mysql::{Pool, OptsBuilder};
use std::env;
use once_cell::sync::OnceCell;

static DB_POOL: OnceCell<Pool> = OnceCell::new();

pub fn init_pool() -> Result<(), mysql::Error> {
    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(env::var("DB_HOST").expect("DB_HOST must be set")))
        .user(Some(env::var("DB_USERNAME").expect("DB_USERNAME must be set")))
        .pass(Some(env::var("DB_PASSWORD").expect("DB_PASSWORD must be set")))
        .db_name(Some(env::var("DB_DATABASE").expect("DB_DATABASE must be set")))
        .tcp_port(
            env::var("DB_PORT")
                .expect("DB_PORT must be set")
                .parse::<u16>()
                .expect("Failed to parse DB_PORT as a number"),
        );
    
    let pool = Pool::new(opts)?;
    DB_POOL.set(pool).expect("Failed to initialize DB pool");
    Ok(())
}

pub fn get_pool() -> &'static Pool {
    DB_POOL.get().expect("Database pool not initialized")
}
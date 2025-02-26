use serde::{Deserialize, Serialize};
use mysql::prelude::Queryable;
use crate::db::get_pool;

#[derive(Debug, Serialize, Deserialize)]
pub struct Url {
    pub id: u64,
    pub short_name: String,
    pub original_url: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUrlRequest {
    pub original_url: String,
}

#[derive(Debug, Serialize)]
pub struct CreateUrlResponse {
    pub short_name: String,
    pub short_url: String,
}

impl Url {
    pub fn create(original_url: String, short_name: String) -> Result<Self, mysql::Error> {
        let pool = get_pool();
        let mut conn = pool.get_conn()?;
        
        let query = "INSERT INTO urls (short_name, original_url) VALUES (?, ?)";
        conn.exec_drop(query, (&short_name, &original_url))?;
        
        // 获取最后插入的 ID
        let id = conn.exec_first("SELECT LAST_INSERT_ID()", ())?.unwrap();
        
        Ok(Self {
            id,
            short_name,
            original_url,
        })
    }
    
    pub fn find_by_short_name(short_name: &str) -> Result<Option<String>, mysql::Error> {
        let pool = get_pool();
        let mut conn = pool.get_conn()?;
        let query = "SELECT original_url FROM urls WHERE short_name = ?";
        conn.exec_first(query, (short_name,))
    }

    pub fn find_all() -> Result<Vec<Self>, mysql::Error> {
        let pool = get_pool();
        let mut conn = pool.get_conn()?;
        let query = "SELECT id, short_name, original_url FROM urls";
        let urls = conn.query_map(
            query,
            |(id, short_name, original_url)| Self {
                id,
                short_name,
                original_url,
            },
        )?;
        Ok(urls)
    }
}
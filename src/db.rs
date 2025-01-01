// src/db.rs
use mongodb::{Client, Database};
use std::env;
use dotenv::dotenv;
use moka::future::Cache;
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct MongoRepo {
    client: Client,
    cache: Cache<String, Arc<Database>>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = env::var("MONGODB_URI").expect("MONGODB_URI not set");
        let client = Client::with_uri_str(&uri)
            .await
            .expect("Failed to initialize client.");

        let cache = Cache::builder()
            .time_to_idle(Duration::from_secs(15 * 60))
            .build();

        MongoRepo { client, cache }
    }

    pub async fn get_db(&self, db_name: &str) -> Arc<Database> {
        if let Some(db) = self.cache.get(db_name) {
            db
        } else {
            let db = Arc::new(self.client.database(db_name));
            self.cache.insert(db_name.to_string(), db.clone()).await;
            db
        }
    }
}

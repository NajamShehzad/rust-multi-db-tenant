use mongodb::{Client, Database};
use std::env;
use dotenv::dotenv;
use moka::future::Cache;
use std::sync::Arc;
use std::time::Duration;
use log::{info};

#[derive(Clone)]
pub struct MongoRepo {
    client: Client,
    cache: Cache<String, Arc<Database>>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = env::var("MONGODB_URI").expect("MONGODB_URI not set");

        info!("Initializing MongoDB client...");
        let client = Client::with_uri_str(&uri)
            .await
            .expect("Failed to initialize client.");
        info!("MongoDB client initialized successfully.");

        let cache = Cache::builder()
            .time_to_idle(Duration::from_secs(15 * 60)) // 15 minutes idle expiration
            .build();

        info!("Cache initialized with a 15-minute TTL.");

        MongoRepo { client, cache }
    }

    pub async fn get_db(&self, db_name: &str) -> Arc<Database> {
        // Log the database name being requested
        info!("Requested database: {}", db_name);

        if let Some(db) = self.cache.get(db_name) {
            // Log cache hit
           return db
        } else {
            // Log cache miss
            info!("Cache miss for database: {}", db_name);
            let db = Arc::new(self.client.database(db_name));
            self.cache.insert(db_name.to_string(), db.clone()).await;

            // Log that the database is inserted into the cache
            info!("Database {} inserted into the cache.", db_name);

            return db
        }
    }
}

use mongodb::Client;

pub mod bigrams;
pub mod layouts;

#[derive(Clone)]
pub struct MongoRepo {
    pub layouts: layouts::LayoutRepo,
    pub bigrams: bigrams::BigramRepo,
}

impl MongoRepo {
    pub async fn init(db_name: &str) -> Self {
        let mongo_uri = std::env::var("MONGO_URI").expect("MONGO_URI must be set");

        let client = Client::with_uri_str(&mongo_uri)
            .await
            .expect("Failed to initialize client.");
        let db = client.database(db_name);

        let layouts = layouts::LayoutRepo::init(&db).await;
        let bigrams = bigrams::BigramRepo::init(&db).await;

        Self {
            layouts,
            bigrams,
        }
    }

    #[allow(dead_code)]
    pub async fn drop(db_name: &str) {
        let mongo_uri = std::env::var("MONGO_URI").expect("MONGO_URI must be set");

        let client = Client::with_uri_str(&mongo_uri)
            .await
            .expect("Failed to initialize client.");
        let db = client.database(db_name);

        db.drop(None).await.expect("Failed to drop database.");
    }
}

use mongodb::Client;

pub mod greetings;

#[derive(Clone)]
pub struct MongoRepo {
    pub greetings: greetings::GreetingRepo,
}

impl MongoRepo {
    pub async fn init(db_name: &str) -> Self {
        let mongo_uri = std::env::var("MONGO_URI").expect("MONGO_URI must be set");

        let client = Client::with_uri_str(&mongo_uri)
            .await
            .expect("Failed to initialize client.");
        let db = client.database(db_name);

        let greetings = greetings::GreetingRepo::init(&db).await;

        Self { greetings }
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

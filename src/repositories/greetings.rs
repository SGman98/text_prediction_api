use bson::doc;
use mongodb::error::Error;

use crate::models::greetings::GreetingModel;

#[derive(Clone)]
pub struct GreetingRepo {
    pub collection: mongodb::Collection<GreetingModel>,
}

impl GreetingRepo {
    pub async fn init(db: &mongodb::Database) -> Self {
        let collection = db.collection::<GreetingModel>("greeting_users");

        Self { collection }
    }
    pub async fn greet(&self, username: &str) -> Result<Option<GreetingModel>, Error> {
        let filter = doc! { "username": username };
        let update = doc! { "$inc": { "count": 1 } };
        let options = mongodb::options::FindOneAndUpdateOptions::builder()
            .upsert(true)
            .build();

        self.collection
            .find_one_and_update(filter, update, options)
            .await
    }
}

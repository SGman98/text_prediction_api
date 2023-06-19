use bson::doc;
use futures::stream::TryStreamExt;
use mongodb::{
    error::Error,
    options::{FindOptions, IndexOptions, UpdateOptions},
    results, IndexModel,
};

use crate::models::{bigrams::BigramModel, pagination::Pagination};

#[derive(Clone)]
pub struct BigramRepo {
    pub collection: mongodb::Collection<BigramModel>,
}

impl BigramRepo {
    pub async fn init(db: &mongodb::Database) -> Self {
        let options = IndexOptions::builder().unique(true).build();
        let model = IndexModel::builder()
            .keys(doc! { "first": 1, "second": 1 })
            .options(options)
            .build();
        let collection = db.collection::<BigramModel>("bigrams");

        collection
            .create_index(model, None)
            .await
            .expect("Failed to create index on bigrams collection.");

        Self { collection }
    }

    pub async fn upsert(&self, first: &str, second: &str) -> Result<results::UpdateResult, Error> {
        let filter = doc! {"first": first, "second": second};
        let update = doc! {"$inc": {"count": 1}};
        let options = UpdateOptions::builder().upsert(true).build();
        self.collection.update_one(filter, update, options).await
    }

    pub async fn find_all(&self, pagination: Pagination) -> Result<Vec<BigramModel>, Error> {
        let options = FindOptions::builder()
            .sort(doc! {"count": -1})
            .limit(pagination.limit.unwrap_or(10))
            .skip(pagination.offset.unwrap_or(0))
            .build();
        self.collection
            .find(None, options)
            .await?
            .try_collect()
            .await
    }
}

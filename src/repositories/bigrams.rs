use bson::doc;
use futures::stream::TryStreamExt;
use mongodb::{
    error::Error,
    options::{FindOptions, IndexOptions, UpdateOptions},
    results, IndexModel,
};

use crate::{
    models::{
        bigrams::{BigramModel, Prediction},
        pagination::Pagination,
    },
    utils::get_regex,
};

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

    pub async fn find_predictions(
        &self,
        first: Option<&str>,
        second: Option<&str>,
        keys: Vec<String>,
    ) -> Result<Vec<Prediction>, Error> {
        let (first, second) = match (first, second) {
            (Some(first), Some(second)) => (
                format!("^{}$", first),
                format!("^{}", get_regex(second, keys)),
            ),
            (Some(first), None) => (format!("^{}$", first), "^.*".to_string()),
            (None, Some(second)) => ("^.*".to_string(), format!("^{}", get_regex(second, keys))),
            (None, None) => ("^.*".to_string(), "^.*".to_string()),
        };

        let pipeline = vec![
            doc! {"$match": {"first": {"$regex": &first} , "second": {"$regex": &second}}},
            doc! {"$group": {"_id": null, "total": {"$sum": "$count"}}},
        ];

        let total_count = self
            .collection
            .aggregate(pipeline, None)
            .await?
            .try_next()
            .await?
            .unwrap_or(doc! {"total": 0})
            .get_i32("total")
            .unwrap();

        let pipeline = vec![
            doc! {"$match": {"first": {"$regex": first} , "second": {"$regex": second}}},
            doc! {"$group": {"_id": "$second", "count": {"$sum": "$count"}}},
            doc! {"$project": {"_id": 0, "word": "$_id", "probability": {"$divide": ["$count", total_count]}}},
            doc! {"$sort": {"probability": -1}},
        ];

        let mut result = self.collection.aggregate(pipeline, None).await?;

        let mut predictions = vec![];
        while let Some(doc) = result.try_next().await? {
            let doc: Prediction = bson::from_document(doc)?;
            predictions.push(doc);
        }
        Ok(predictions)
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

use bson::doc;
use futures::stream::TryStreamExt;
use mongodb::{ error::Error, options::IndexOptions, results, IndexModel};

use crate::models::layouts::LayoutModel;

#[derive(Clone)]
pub struct LayoutRepo {
    pub collection: mongodb::Collection<LayoutModel>,
}

impl LayoutRepo {
    pub async fn init(db: &mongodb::Database) -> Self {
        let options = IndexOptions::builder().unique(true).build();
        let model = IndexModel::builder()
            .keys(doc! { "name": 1 })
            .options(options)
            .build();
        let collection = db.collection::<LayoutModel>("layouts");

        collection
            .create_index(model, None)
            .await
            .expect("Failed to create index on layouts collection.");

        Self { collection }
    }

    pub async fn find_all(&self) -> Result<Vec<LayoutModel>, Error> {
        self.collection.find(None, None).await?.try_collect().await
    }

    pub async fn create(&self, layout: &LayoutModel) -> Result<results::InsertOneResult, Error> {
        self.collection.insert_one(layout, None).await
    }

    pub async fn find(&self, name: &str) -> Result<Option<LayoutModel>, Error> {
        self.collection.find_one(doc! {"name": name}, None).await
    }

    pub async fn update(
        &self,
        name: &str,
        layout: &LayoutModel,
    ) -> Result<Option<LayoutModel>, Error> {
        self.collection
            .find_one_and_replace(doc! {"name": name}, layout, None)
            .await
    }

    pub async fn delete(&self, name: &str) -> Result<Option<LayoutModel>, Error> {
        self.collection
            .find_one_and_delete(doc! {"name": name}, None)
            .await
    }
}

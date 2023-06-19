use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BigramModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<mongodb::bson::oid::ObjectId>,
    pub first: String,
    pub second: String,
    pub count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessTextRequest {
    pub text: String,
}

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

trait Task {
    fn new() -> Self;
    fn update(&self) -> Self;
    fn delete(&self) -> Self;
    fn get(&self) -> Self;
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Track {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    pub title: String,
    pub artist: String,
    pub description: String,
    pub published: bool,
    pub updated: String,
    pub created: String,
}

pub struct Tracks {
    pub tracks: Vec<Track>,
}

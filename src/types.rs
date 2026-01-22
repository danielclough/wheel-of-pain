use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub title: String,
    pub claim: Option<String>,
    pub description: Option<String>,
    pub category: Vec<String>,
    pub episode: Option<EpisodeNumber>,
    pub url: Option<UrlOrVec>,
    pub drink: Option<String>,
    pub sponsor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EpisodeNumber {
    Single(u32),
    Multiple(Vec<f32>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UrlOrVec {
    Single(String),
    Multiple(Vec<String>),
}

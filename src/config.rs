use std::collections::HashMap;
use serde_derive::Deserialize;
use std::sync::atomic::AtomicU64;
use rocket::tokio::sync::RwLock;

#[derive(Deserialize, Debug)]
pub struct UploadConfig {
    pub settings: UploadSettings,
    pub user: HashMap<String, UploadUser>,
}

#[derive(Deserialize, Debug)]
pub struct UploadSettings {
    pub output: String,
    pub min_id: Option<String>,
    pub max_id: Option<String>,
    pub upload_limit: Option<String>,
    pub log: Option<String>,
    pub timestamp_format: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct UploadUser {
    pub token: String,
    pub descriptor: Option<String>,
    pub upload_limit: Option<String>,
    pub timestamp_format: Option<String>,
}

#[derive(Debug)]
pub struct UploadState {
    pub output: RwLock<String>,
    pub lower: AtomicU64,
    pub upper: AtomicU64,
    pub log: RwLock<String>,
    pub timestamp_format: RwLock<String>,
}
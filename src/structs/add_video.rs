use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddVideoResponse {
    pub data: AddVideoData,
    pub errcode: i32,
    pub message: String,
    pub front_version: String,
    pub update_message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddVideoData {
    pub log: AddvideoLog,
    pub is_warning: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddvideoLog {
    pub r#type: i32,
    pub user_id: i64,
    pub admin_id: i32,
    pub price: f64,
    pub video_id: String,
    pub video_url: String,
    pub video_cover: String,
    pub video_author: String,
    pub region: String,
    pub likes: i64,
    pub resource_id: Option<String>,
    pub extral_profit: Option<String>,
    pub status: i32,
    pub updated_at: String,
    pub created_at: String,
    pub id: i64,
}

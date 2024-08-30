use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddCommentResponse {
    pub data: AddCommentData,
    pub errcode: i32,
    pub message: String,
    pub front_version: String,
    pub update_message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddCommentData {
    pub log: AddCommentLog,
    pub is_warning: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddCommentLog {
    #[serde(rename = "type")]
    pub log_type: i32,
    pub user_id: i32,
    pub admin_id: i32,
    pub price: f64,
    pub video_id: String,
    pub video_url: String,
    pub video_cover: String,
    pub video_author: String,
    pub region: String,
    pub comments: i32,
    pub likes: i32,
    pub talk: String,
    pub resource_id: Option<serde_json::Value>,
    pub extral_profit: Option<serde_json::Value>,
    pub status: i32,
    pub updated_at: String,
    pub created_at: String,
    pub id: i64,
}

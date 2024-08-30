use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SubmitResponse {
    pub data: SubmitData,
    pub errcode: i32,
    pub message: String,
    pub front_version: String,
    pub update_message: String,
}

#[derive(Debug, Deserialize)]
pub struct SubmitData {
    pub log: SubmitLog,
}

#[derive(Debug, Deserialize)]
pub struct SubmitLog {
    pub id: i64,
    pub r#type: i32,
    pub user_id: i64,
    pub admin_id: i32,
    pub price: String,
    pub video_id: String,
    pub video_url: String,
    pub video_cover: String,
    pub video_author: String,
    pub likes: i64,
    pub current_likes: Option<i64>,
    pub comments: Option<i64>,
    pub current_comments: Option<i64>,
    pub talk: Option<String>,
    pub region: String,
    pub status: i32,
    pub resource_id: Option<String>,
    pub extral_profit: Option<f64>,
    pub created_at: String,
    pub clicked_at: String,
    pub updated_at: String,
    pub submit_at: String,
    pub completed_at: Option<String>,
}

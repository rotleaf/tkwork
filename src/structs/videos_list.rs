use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VideosListResponse {
    pub data: VideosListData,
    pub errcode: i32,
    pub message: String,
    pub front_version: String,
    pub update_message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideosListData {
    pub list: Vec<VideoInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoInfo {
    pub id: i64,
    pub video_id: String,
    pub author: String,
    pub video_title: String,
    pub video_url: String,
    pub video_likes: i64,
    pub video_cover: String,
    pub video_comments: i64,
    pub region: String,
    pub status: i32,
    pub vstatus: i32,
    pub created_at: String,
    pub updated_at: String,
    pub created_at_time: String,
    pub price: String,
}

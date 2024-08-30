use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentListResponse {
    pub data: CommentListData,
    pub errcode: i32,
    pub message: String,
    pub front_version: String,
    pub update_message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentListData {
    pub list: Vec<CommentListVideoItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentListVideoItem {
    pub id: i32,
    pub video_id: String,
    pub author: String,
    pub video_title: String,
    pub video_url: String,
    pub video_likes: i32,
    pub video_cover: String,
    pub video_comments: i32,
    pub region: String,
    pub status: i32,
    pub vstatus: i32,
    pub created_at: String,
    pub updated_at: String,
    pub price: String,
}

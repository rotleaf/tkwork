use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountInfoResponse {
    pub data: AccountInfoData,
    pub errcode: i32,
    pub message: String,
    pub front_version: String,
    pub update_message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountInfoData {
    pub account_info: AccountInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountInfo {
    pub id: i64,
    pub address: Option<String>,
    pub privatekey: Option<String>,
    pub username: String,
    pub mobile: String,
    pub pid: i64,
    pub unread_num: i32,
    pub avatar: String,
    pub invite_code: String,
    pub register_no: i32,
    pub invite_download_url: String,
    pub membership: i32,
    pub robot_status: i32,
    pub mobile_code: String,
    pub email: String,
    pub telegram: String,
    pub whatsapp: String,
    pub withdraw_address: String,
    pub service: String,
    pub points: String,
    pub quantify_amount: String,
    pub member_expired: i32,
    pub is_risk: i32,
    pub is_warning: i32,
    pub resource_id: i32,
    pub is_test: i32,
    pub tiktok_open_id: String,
    pub is_contact: i32,
}

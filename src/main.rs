pub mod api;
pub mod structs;
pub mod utils;

use api::account::account_actions::account_info;
use api::comment_list::functions::comment_list;
use api::videos_list::functions::get_video_list;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let _ = account_info().await;
    // perform likes
    let _ = get_video_list().await;
    // perform comments
    let _ = comment_list().await;
}

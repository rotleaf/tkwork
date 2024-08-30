pub mod api;
pub mod structs;
pub mod utils;

use api::account::account_actions::account_info;
use api::comment_list::functions::comment_list;
// use api::videos_list::functions::get_video_list;

// use clap::Parser;
use dotenv::dotenv;

// #[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
// struct Args {
//     /// Name of the person to greet
//     #[arg(short, long)]
//     name: String,

//     /// Number of times to greet
//     #[arg(short, long, default_value_t = 1)]
//     count: u8,
// }

#[tokio::main]
async fn main() {
    dotenv().ok();
    let _ = account_info().await;
    // perform likes
    // let _ = get_video_list().await;
    // perform comments
    let _ = comment_list().await;
}

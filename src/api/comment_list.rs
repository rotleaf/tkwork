pub mod functions {
    use std::{collections::HashMap, env, process, time::Duration};

    use colored::Colorize;
    use reqwest::{Client, Response};
    use tokio::time::sleep;

    use crate::{
        structs::{
            add_comment::AddCommentResponse, comment_list::CommentListResponse,
            submit_task::SubmitResponse,
        },
        utils::global_utils::{get_url, headers},
    };

    pub async fn submit(id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = get_url("videoTaskSubmit");
        let token: String = env::var("USER_TOKEN").expect("USER_TOKEN not set");
        let mut form: HashMap<&str, &str> = HashMap::new();
        form.insert("id", id);
        form.insert("_user_token", &token);
        form.insert("_lang", "en");
        let client: Client = Client::new();

        let response: Response = client
            .post(url)
            .headers(headers())
            .form(&form)
            .send()
            .await?;

        match response.text().await {
            Ok(ok) => match serde_json::from_str::<SubmitResponse>(&ok) {
                Ok(resp) => {
                    if resp.message == "Success" {
                        let earned = resp.data.log.price;
                        println!("*[ earned : ${} ]", earned.bold().blue())
                    } else {
                        println!("* Server error submitting, [weird]")
                    }
                }
                Err(e) => println!("* Error deserializing json: {e}"),
            },
            Err(e) => println!("* Error : {}", e.to_string().bold().red()),
        }
        Ok(())
    }

    pub async fn video_click(id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = get_url("videoClick");
        let token: String = env::var("USER_TOKEN").expect("USER_TOKEN not set");
        let mut form: HashMap<&str, &str> = HashMap::new();
        form.insert("id", id);
        form.insert("_user_token", &token);
        form.insert("_lang", "en");
        let client: Client = Client::new();

        let response: Response = client
            .post(url)
            .headers(headers())
            .form(&form)
            .send()
            .await?;

        match response.text().await {
            Ok(ok) => {
                if ok.contains("\"message\":\"Success\"") {
                    println!("* Clicked");
                } else {
                    println!("* Server error clicking link, [weird]")
                }
            }
            Err(e) => println!("* Error : {}", e.to_string().bold().red()),
        }
        Ok(())
    }

    pub async fn add_comment_video(id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = get_url("addVideoComment");
        let token: String = env::var("USER_TOKEN").expect("USER_TOKEN not set");
        let mut form: HashMap<&str, &str> = HashMap::new();
        form.insert("id", id);
        form.insert("_user_token", &token);
        form.insert("_lang", "en");
        let client: Client = Client::new();

        let response: Response = client
            .post(url)
            .headers(headers())
            .form(&form)
            .send()
            .await?;
        match response.text().await {
            Ok(ok) => match serde_json::from_str::<AddCommentResponse>(&ok) {
                Ok(resp) => {
                    if resp.message == "Success" {
                        let id = resp.data.log.id;
                        let _ = video_click(id.to_string().as_str()).await;
                        sleep(Duration::from_secs(30)).await;
                        let _ = submit(id.to_string().as_str()).await;
                        println!("{}", "#".repeat(40));
                        sleep(Duration::from_secs(25)).await;
                    } else {
                        println!("* Error from server: {}", resp.message);
                    }
                }
                Err(_) => {
                    if ok.contains("In order to prevent") {
                        println!("* Maximum reached, rest");
                        process::exit(0);
                    } else if ok.contains("Your request is too frequent") {
                        println!("* I don't know how it got here, stopping");
                        process::exit(0);
                    }
                }
            },
            Err(e) => println!("* Error : {}", e.to_string().bold().red()),
        }
        Ok(())
    }
    pub async fn comment_list() -> Result<(), Box<dyn std::error::Error>> {
        let url = get_url("getCommentList");
        let token: String = env::var("USER_TOKEN").expect("USER_TOKEN not set");
        let mut form: HashMap<&str, &str> = HashMap::new();
        form.insert("page", "1");
        form.insert("_user_token", &token);
        form.insert("_lang", "en");
        let client: Client = Client::new();

        let response: Response = client
            .post(url)
            .headers(headers())
            .form(&form)
            .send()
            .await?;

        match response.text().await {
            Ok(ok) => match serde_json::from_str::<CommentListResponse>(&ok) {
                Ok(resp) => {
                    if resp.message == "Success" {
                        let videos = resp.data.list;
                        for video in videos {
                            let video_id = video.id;
                            let _ = add_comment_video(video_id.to_string().as_str()).await?;
                            // break;
                        }
                    } else {
                        println!("* Error from server: {}", resp.message);
                    }
                }
                Err(e) => println!("* Error deserializing json: {e}"),
            },
            Err(e) => println!("* Error : {}", e.to_string().bold().red()),
        }
        Ok(())
    }
}

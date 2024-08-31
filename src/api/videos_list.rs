pub mod functions {
    use std::{collections::HashMap, env, process, time::Duration};

    use colored::Colorize;
    use reqwest::{Client, Response};
    use tokio::time::sleep;

    use crate::{
        structs::{
            add_video::AddVideoResponse, submit_task::SubmitResponse,
            videos_list::VideosListResponse,
        },
        utils::global_utils::{get_url, headers},
    };

    pub async fn submit_video(id: &str) -> Result<(), Box<dyn std::error::Error>> {
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
                        let earned: String = resp.data.log.price;
                        println!("*[ earned : ${} ]", earned.bold().blue())
                    } else {
                        println!("* Error from server: {}", resp.message);
                    }
                }
                Err(e) => println!("* Error: {e}"),
            },
            Err(e) => println!("* Error: {e}"),
        }
        Ok(())
    }

    pub async fn click_link(id: &str) -> Result<(), Box<dyn std::error::Error>> {
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
            Err(e) => println!("* Error : {e}"),
        }
        Ok(())
    }
    pub async fn add_video(id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = get_url("addVideoTask");
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
            Ok(ok) => match serde_json::from_str::<AddVideoResponse>(&ok) {
                Ok(resp) => {
                    if resp.message == "Success" {
                        println!("* Video added to queue");
                        let id: i64 = resp.data.log.id;
                        let _ = click_link(id.to_string().as_str()).await;
                        // continue but sleep a while, 30 seconds
                        sleep(Duration::from_secs(30)).await;
                        let _ = submit_video(id.to_string().as_str()).await;
                    } else {
                        println!("* Error from server: {}", resp.message);
                    }
                }
                Err(_) => {
                    if ok.contains("In order to prevent") {
                        println!("* Comment tasks complete");
                        // process::exit(0);
                    } else if ok.contains("The account has been disabled") {
                        println!("* Account disabled");
                        process::exit(0);
                    }
                }
            },
            Err(e) => println!("* Error - {e}"),
        }

        Ok(())
    }

    pub async fn get_video_list() -> Result<(), Box<dyn std::error::Error>> {
        let url = get_url("getVideoList");
        let user_token: String = env::var("USER_TOKEN").expect("USER_TOKEN not set");
        let mut form: HashMap<&str, &str> = HashMap::new();
        form.insert("page", "1");
        form.insert("_user_token", user_token.as_str());
        form.insert("_lang", "en");
        let client: Client = Client::new();

        let response: Response = client
            .post(url)
            .headers(headers())
            .form(&form)
            .send()
            .await?;

        match response.text().await {
            Ok(ok) => match serde_json::from_str::<VideosListResponse>(&ok) {
                Ok(resp) => {
                    if resp.message == "Success" {
                        let videos = resp.data.list;
                        for video in videos {
                            // let price = video.price;
                            // let video_url = video.video_url;
                            let id = video.id;
                            let _ = add_video(id.to_string().as_str()).await?;
                            // break;
                            println!("{}", "#".repeat(40));
                            sleep(Duration::from_secs(20)).await;
                        }
                    } else {
                        println!("* Error from server: {}", resp.message);
                    }
                }
                Err(e) => println!("* Error deserializing json: {e}"),
            },
            Err(e) => println!("* Error - {e}"),
        }
        Ok(())
    }
}

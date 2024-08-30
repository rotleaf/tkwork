pub mod account_actions {
    use std::{collections::HashMap, env};

    use colored::Colorize;
    use reqwest::{Client, Response};

    use crate::{structs::account_info::AccountInfoResponse, utils::global_utils::headers};

    //account info
    pub async fn account_info() -> Result<(), Box<dyn std::error::Error>> {
        let user_token: String = env::var("USER_TOKEN").expect("USER_TOKEN is not set");
        let url: &str = "https://www.tkwork.org/api/app.user/account/info";
        let mut form: HashMap<&str, &str> = HashMap::new();
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
            Ok(ok) => match serde_json::from_str::<AccountInfoResponse>(&ok) {
                Ok(resp) => {
                    if resp.message == "Success" {
                        let balance: String = resp.data.account_info.quantify_amount;
                        let points_today: String = resp.data.account_info.points;
                        println!(
                            " Balance - {}\n   Points Today - {}",
                            balance.bold().green(),
                            points_today.bold().green()
                        );
                        println!("{}", "#".repeat(40));
                    } else {
                        println!("* Server message not ok");
                    }
                }
                Err(e) => println!("* Error deserializing json: {}", e.to_string().bold().red()),
            },
            Err(e) => println!("* Error: {}", e.to_string().bold().red()),
        }
        Ok(())
    }
}

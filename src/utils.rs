pub mod global_utils {
    use std::env;

    use reqwest::header::{
        HeaderMap, HeaderValue, CONNECTION, COOKIE, ORIGIN, REFERER, USER_AGENT,
    };

    pub fn headers() -> HeaderMap {
        let user_agent = env::var("USER_AGENT").expect("USER_AGENT not set");
        let cookie = env::var("COOKIE").expect("COOKIE is not set");
        let mut headers = HeaderMap::new();

        headers.insert(USER_AGENT, HeaderValue::from_str(&user_agent).unwrap());
        headers.insert(COOKIE, HeaderValue::from_str(&cookie).unwrap());
        headers.insert(ORIGIN, HeaderValue::from_str("https://tkwork.org").unwrap());
        headers.insert(
            REFERER,
            HeaderValue::from_str("https://tkwork.org/").unwrap(),
        );
        headers.insert(CONNECTION, HeaderValue::from_str("keep-alive").unwrap());

        headers
    }

    pub fn get_url(path: &str) -> String {
        let link = format!("https://www.tkwork.org/api/app.quantize/task/{}", path);
        link
    }
}

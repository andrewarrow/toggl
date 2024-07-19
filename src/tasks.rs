use reqwest::header::{CONTENT_TYPE, COOKIE};
use std::env;

pub async fn fetch_tasks() -> Result<(), Box<dyn std::error::Error>> {
    let toggl_cookie = env::var("TOGGL_COOKIE").expect("TOGGL_COOKIE environment variable not set");

    let client = reqwest::Client::new();
    let response = client
        .get("https://track.toggl.com/api/v9/me/tasks?meta=true")
        .header(CONTENT_TYPE, "application/javascript")
        .header(COOKIE, toggl_cookie)
        .send()
        .await?;

    if response.status().is_success() {
        let text = response.text().await?;
        println!("{}", text);
    } else {
        eprintln!("Request failed with status: {}", response.status());
    }

    Ok(())
}

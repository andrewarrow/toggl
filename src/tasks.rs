use reqwest::header::{CONTENT_TYPE, COOKIE};
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct Task {
    id: u64,
    name: String,
    project_id: Option<u64>,
}

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
        let tasks: Vec<Task> = response.json().await?;
        println!("{:#?}", tasks);
    } else {
        eprintln!("Request failed with status: {}", response.status());
    }

    Ok(())
}

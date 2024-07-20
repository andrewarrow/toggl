use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
use std::str::FromStr;

#[derive(Serialize)]
struct TimeData {
    created_with: String,
    pid: u32,
    tid: u32,
    start: String,
    stop: String,
    wid: u32,
    duration: u32,
    description: String,
    billable: bool,
    tags: Vec<String>,
    project_name: String,
    project_color: String,
    project_active: bool,
    client_name: String,
    project_billable: bool,
}

pub async fn post_request() -> reqwest::Result<reqwest::Response> {
    let post_data = TimeData {
        created_with: "Snowball".to_string(),
        pid: 123,
        tid: 1234,
        start: "2024-07-18T16:00:00.000Z".to_string(),
        stop: "2024-07-19T00:00:00.000Z".to_string(),
        wid: 7000311,
        duration: 28800,
        description: "ratpack".to_string(),
        billable: false,
        tags: vec![],
        project_name: "foo Booking App".to_string(),
        project_color: "#566614".to_string(),
        project_active: true,
        client_name: "foo".to_string(),
        project_billable: false,
    };

    let client = reqwest::Client::new();
    let url = "https://track.toggl.com/api/v9/time_entries?meta=true";

    let cookie = env::var("TOGGL_COOKIE").expect("TOGGL_COOKIE must be set");
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Cookie", cookies);

    client
        .post(url)
        .headers(headers)
        .json(&post_data)
        .send()
        .await
}

//use serde::{Deserialize, Serialize};
use serde::Serialize;
//use serde_json::json;
use std::env;
use std::error::Error;
//use std::str::FromStr;

#[derive(Serialize)]
struct TimeData {
    created_with: String,
    pid: u64,
    tid: u64,
    start: String,
    stop: String,
    wid: u64,
    duration: u64,
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
    let taskIdStr = env::var("TOGGLE_TASK_ID").expect("TOGGLE_TASK_ID must be set");
    let tid: u64 = taskIdStr.parse().unwrap();

    let content = tasks::read_file(format!("data/task{}.json", taskIdStr));

    let m = serde_json::from_str(content);
    m.get("project_id");

    let post_data = TimeData {
        created_with: "Snowball".to_string(),
        pid: m.get("project_id"),
        tid: tid,
        start: "2024-07-18T16:00:00.000Z".to_string(),
        stop: "2024-07-19T00:00:00.000Z".to_string(),
        wid: m.get("workspace_id"),
        duration: 3600,
        description: "coding".to_string(),
        billable: false,
        tags: vec![],
        project_name: m.get("project_name"),
        project_color: m.get("project_color"),
        project_active: true,
        client_name: m.get("client_name"),
        project_billable: false,
    };

    let client = reqwest::Client::new();
    let url = "https://track.toggl.com/api/v9/time_entries?meta=true";

    let cookie = env::var("TOGGL_COOKIE").expect("TOGGL_COOKIE must be set");
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Cookie", cookie.parse().unwrap());

    client
        .post(url)
        .headers(headers)
        .json(&post_data)
        .send()
        .await
}

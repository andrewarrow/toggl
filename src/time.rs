//use serde::{Deserialize, Serialize};
use serde::Serialize;
//use serde_json::json;
use std::env;
use std::error::Error;
//use std::str::FromStr;
use std::fs;
//use std::fs::File;
//use std::io::Read;
//use std::path::Path;
use serde_json::Value;
//use std::collections::HashMap;
//use super::tasks;

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

    let file_path = format!("data/task{}.json", taskIdStr);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let result: Result<Value, serde_json::Error> = serde_json::from_str(&contents);

    //let m: tasks::Task = serde_json::from_str(&contents).unwrap();

    let m: Value = match result {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Error deserializing JSON: {}", e);
            serde_json::Value::Object(serde_json::Map::new())
        }
    };

    let pid: Option<u64> = m.get("project_id").and_then(|v| v.as_u64());
    let pid = match pid {
        Some(id) => id,
        None => 0,
    };
    let wid: Option<u64> = m.get("workspace_id").and_then(|v| v.as_u64());
    let wid = match wid {
        Some(id) => id,
        None => 0,
    };
    let project_name: Option<&str> = m.get("project_name").and_then(|v| v.as_str());
    let project_name = match project_name {
        Some(id) => id,
        None => "",
    };
    let project_color: Option<&str> = m.get("project_color").and_then(|v| v.as_str());
    let project_color = match project_color {
        Some(id) => id,
        None => "",
    };
    let client_name: Option<&str> = m.get("client_name").and_then(|v| v.as_str());
    let client_name = match client_name {
        Some(id) => id,
        None => "",
    };

    let post_data = TimeData {
        created_with: "Snowball".to_string(),
        pid: pid,
        tid: tid,
        start: "2024-07-18T16:00:00.000Z".to_string(),
        stop: "2024-07-19T00:00:00.000Z".to_string(),
        wid: wid,
        duration: 3600,
        description: "coding".to_string(),
        billable: false,
        tags: vec![],
        project_name: project_name.to_string(),
        project_color: project_color.to_string(),
        project_active: true,
        client_name: client_name.to_string(),
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

use serde::Serialize;
use serde_json::Value;
use std::env;
use std::error::Error;
use std::fs;
//use super::tasks;

#[derive(Serialize, Debug)]
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

pub async fn post_request(t1: String, t2: String) -> reqwest::Result<reqwest::Response> {
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
        start: t1.to_string(),
        stop: t2.to_string(),
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

    println!("{:#?}", post_data);

    let client = reqwest::Client::new();
    //let url = "https://track.toggl.com/api/v9/time_entries?meta=true";
    let url = "http://localhost:3000";

    let cookie = match env::var("TOGGL_COOKIE") {
        Ok(v) => v,
        Err(_) => String::new(),
    };
    let token = match env::var("TOGGL_TOKEN") {
        Ok(v) => v,
        Err(_) => String::new(),
    };

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    if cookie.is_empty() {
        let basic = format!("{}:api_token", token);
        headers.insert("Authorization: Basic", basic.parse().unwrap());
    } else {
        headers.insert("Cookie", cookie.parse().unwrap());
    }

    client
        .post(url)
        .headers(headers)
        .json(&post_data)
        .send()
        .await
}

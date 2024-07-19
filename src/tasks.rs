use reqwest::header::{CONTENT_TYPE, COOKIE};
use serde::Deserialize;
use std::collections::HashMap;
use std::env;

#[derive(Deserialize, Debug)]
pub struct Task {
    id: u64,
    name: String,
    workspace_id: u64,
    project_id: u64,
    project_name: String,
    client_name: String,
}

pub async fn fetch_tasks() -> Result<HashMap<String, Vec<Task>>, Box<dyn std::error::Error>> {
    let toggl_cookie = env::var("TOGGL_COOKIE").expect("TOGGL_COOKIE environment variable not set");

    let client = reqwest::Client::new();
    let response = client
        .get("https://track.toggl.com/api/v9/me/tasks?meta=true")
        .header(CONTENT_TYPE, "application/javascript")
        .header(COOKIE, toggl_cookie)
        .send()
        .await?;

    if response.status().is_success() {
        let mut tasks_by_project: HashMap<String, Vec<Task>> = HashMap::new();

        let tasks: Vec<Task> = response.json().await?;

        for task in tasks {
            tasks_by_project
                .entry(task.project_name.clone())
                .or_insert_with(Vec::new)
                .push(task);
        }
        Ok(tasks_by_project)
    } else {
        eprintln!("Request failed with status: {}", response.status());
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Request failed",
        )))
    }
}

pub fn search_tasks<'a>(
    tasks_by_project: &'a HashMap<String, Vec<Task>>,
    query: &str,
) -> Vec<&'a Task> {
    let mut results = Vec::new();
    for (project_name, tasks) in tasks_by_project {
        if project_name.contains(query) {
            results.extend(tasks);
        }
    }
    results
}

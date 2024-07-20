use reqwest::header::{CONTENT_TYPE, COOKIE};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;
//use tokio::fs::File;
//use tokio::io::AsyncWriteExt;
//use std::fs;
//use std::io;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    id: u64,
    name: String,
    workspace_id: u64,
    project_id: u64,
    project_name: String,
    client_name: String,
    project_color: String,
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
            let serialized_task = serde_json::to_string(&task)?;
            let filename = format!("data/task{}.json", task.id);
            if let Err(_e) = write_to_file(&filename, &serialized_task).await {}

            let t = Task {
                project_name: format!("{} ({})", task.project_name, task.client_name),
                id: task.id,
                name: task.name,
                workspace_id: task.workspace_id,
                project_id: task.project_id,
                project_color: task.project_color,
                client_name: task.client_name,
            };

            tasks_by_project
                .entry(t.project_name.to_lowercase().clone())
                .or_insert_with(Vec::new)
                .push(t);
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

async fn write_to_file(filename: &str, content: &str) -> std::io::Result<()> {
    let mut file = File::create(filename).map_err(|e| {
        eprintln!("Error creating file: {}", e);
        e
    })?;
    file.write_all(content.as_bytes()).map_err(|e| {
        eprintln!("Error writing to file: {}", e);
        e
    })?;
    Ok(())
}

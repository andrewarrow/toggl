use clap::{Arg, Command};
use clap::{Parser, Subcommand};
use std::env;
use tokio::time::{sleep, Duration};
mod tasks;

#[tokio::main]
async fn main() {
    let matches = Command::new("Toggl CLI")
        .version("1.0")
        .author("Andrew Arrow <aa@andrewarrow.dev>")
        .about("add time to toggle from terminal")
        .arg(
            Arg::new("DESC")
                .help("what you did")
                .required(true)
                .index(1),
        )
        .arg(Arg::new("TASK").help("which task?").required(true).index(2))
        .get_matches();

    let query = matches.get_one::<String>("TASK").expect("TASK is required");
    let tasks_by_project = tasks::fetch_tasks().await?;

    let search_results = tasks::search_tasks(&tasks_by_project, query);

    for task in search_results {
        println!("{:#?}", task);
    }
}

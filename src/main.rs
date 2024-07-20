use clap::{Arg, Command};
use clap::{Parser, Subcommand};
use std::env;
use tokio::time::{sleep, Duration};
mod tasks;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("Toggl CLI")
        .version("1.0")
        .author("Andrew Arrow <aa@andrewarrow.dev>")
        .about("add time to toggle from terminal")
        .arg(
            Arg::new("DESC")
                .help("what you did")
                .required(false)
                .long("desc")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("TASK")
                .help("which task?")
                .required(false)
                .long("task")
                .value_parser(clap::value_parser!(String)),
        )
        .get_matches();

    let queryCap = matches.get_one::<String>("TASK");
    let descTxt = matches.get_one::<String>("DESC");
    if queryCap.trim().is_empty() == "" && descTxt.trim().is_empty() {
        println!("--task=prefix or --desc='desc of what I did'");
        return;
    }

    if queryCap.trim().is_empty() != "" {
        let query_lower = queryCap.to_lowercase();
        let query: &str = &query_lower;

        let tasks_by_project = tasks::fetch_tasks().await?;

        let search_results = tasks::search_tasks(&tasks_by_project, query);

        for task in search_results {
            println!("{:#?}", task);
        }
    }

    Ok(())
}

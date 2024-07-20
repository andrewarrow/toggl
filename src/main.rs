use clap::{Arg, Command};
use clap::{Parser, Subcommand};
use std::env;
use tokio::time::{sleep, Duration};
mod tasks;
use chrono::NaiveDate;
use chrono::{DateTime, Datelike, FixedOffset, TimeZone, Timelike};
use std::str::FromStr;

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
        .arg(
            Arg::new("TIME")
                .help("times like July 19, 9-5 PST")
                .required(false)
                .long("time")
                .value_parser(clap::value_parser!(String)),
        )
        .get_matches();

    let queryCap: Option<String> = matches.get_one("TASK").cloned();
    let default_value = "".to_string();
    let queryCap = queryCap.unwrap_or(default_value);

    let timeTxt: Option<String> = matches.get_one("TIME").cloned();
    let default_value = "".to_string();
    let timeTxt = timeTxt.unwrap_or(default_value);

    let descTxt: Option<String> = matches.get_one("DESC").cloned();
    let default_value = "".to_string();
    let descTxt = descTxt.unwrap_or(default_value);

    if queryCap.trim().is_empty() && descTxt.trim().is_empty() {
        println!("--task=prefix or --desc='desc of what I did'");
        return Ok(());
    }

    if queryCap.trim().is_empty() == false {
        let query_lower = queryCap.to_lowercase();
        let query: &str = &query_lower;

        let tasks_by_project = tasks::fetch_tasks().await?;

        let search_results = tasks::search_tasks(&tasks_by_project, query);

        for task in search_results {
            println!("{:#?}", task);
        }
    }
    if timeTxt.trim().is_empty() == false {
        println!("");

        let input_string = timeTxt.to_string();
        let input: &str = &input_string;
        match NaiveDate::parse_from_str(input, "%Y-%m-%d") {
            Ok(date) => println!("Parsed date: {}", date),
            Err(e) => eprintln!("Failed to parse date: {}", e),
        }
    }

    Ok(())
}

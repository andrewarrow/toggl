use clap::{Arg, Command};
use clap::{Parser, Subcommand};
use std::env;
use tokio::time::{sleep, Duration};
mod tasks;
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

        let year = 2024;

        let offset = FixedOffset::west(8 * 3600);
        let parts: Vec<&str> = timeTxt.split_whitespace().collect();

        let date_str = format!("{} {}", parts[0], parts[1]); // "Jul 19"
        let time_range_str = parts[2]; // "9-5"

        let date = DateTime::parse_from_str(&format!("{} {}", date_str, year), "%b %d %Y").unwrap();

        let time_range_parts: Vec<&str> = time_range_str.split('-').collect();
        let start_hour: u32 = time_range_parts[0].parse().unwrap();
        let end_hour: u32 = time_range_parts[1].parse().unwrap();
        let start_time = date
            .with_hour(start_hour)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap();
        let end_time = date
            .with_hour(end_hour)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap();

        let start_time_pst = start_time.with_timezone(&offset);
        let end_time_pst = end_time.with_timezone(&offset);

        println!("Start Time: {}", start_time_pst);
        println!("End Time: {}", end_time_pst);
    }

    Ok(())
}

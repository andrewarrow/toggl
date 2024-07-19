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
        .get_matches();

    println!("hi");
    tasks::fetch_tasks().await;
    println!("hi");
}

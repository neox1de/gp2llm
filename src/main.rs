mod models;
mod client;

use std::env;
use anyhow::{Result, anyhow};
use client::GitHubClient;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        return Err(anyhow!("Usage: {} <github_username>", args[0]));
    }

    let username = &args[1];
    let token = env::var("GITHUB_TOKEN").ok();
    
    let client = GitHubClient::new(token.as_deref())?;
    
    match client.fetch_all_data(username) {
        Ok(_) => {
            println!("Successfully generated {}.json and {}.md", username, username);
            Ok(())
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}

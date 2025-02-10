use gp2llm::client::GitHubClient;
use std::fs;
use std::path::Path;
use std::env;

fn cleanup_test_files(usernames: &[&str]) {
    for username in usernames {
        let _ = fs::remove_file(format!("{}.json", username));
        let _ = fs::remove_file(format!("{}.md", username));
    }
}

#[test]
fn test_fetch_multiple_users() {
    let token = env::var("GITHUB_TOKEN").ok();
    let client = GitHubClient::new(token.as_deref()).expect("Failed to create client");
    let test_users = ["neox1de", "torvalds", "rust-lang"];
    
    // Clean up any existing test files first
    cleanup_test_files(&test_users);
    
    for username in &test_users {
        let result = client.fetch_all_data(username);
        assert!(result.is_ok(), "Failed to fetch data for {}: {:?}", username, result.err());
        
        // Verify files were created
        assert!(Path::new(&format!("{}.json", username)).exists(), "JSON file missing for {}", username);
        assert!(Path::new(&format!("{}.md", username)).exists(), "MD file missing for {}", username);
    }
    
    // Clean up test files
    cleanup_test_files(&test_users);
}

#[test]
fn test_invalid_user() {
    let token = env::var("GITHUB_TOKEN").ok();
    let client = GitHubClient::new(token.as_deref()).expect("Failed to create client");
    
    let result = client.fetch_all_data("this_user_definitely_does_not_exist_mother_of_father");
    assert!(result.is_err());
}

#[test]
fn test_content_formatting() {
    let token = env::var("GITHUB_TOKEN").ok();
    let client = GitHubClient::new(token.as_deref()).expect("Failed to create client");
    
    // Test with my own user neox1de
    let result = client.fetch_all_data("neox1de");
    assert!(result.is_ok());
    
    // Check if the MD file exists and contains the right formatted data
    let md_content = fs::read_to_string("neox1de.md").expect("Failed to read MD file");
    
    // Cleanup
    cleanup_test_files(&["neox1de"]);
    
    // Verify markdown contains the right sections
    assert!(md_content.contains("# "), "Missing title");
    assert!(md_content.contains("## Overview"), "Missing overview section");
    assert!(md_content.contains("## Repositories"), "Missing repositories section");
    
    // If profile README exists, verify the section exists (but not in a code block)
    if md_content.contains("## Profile README") {
        assert!(md_content.contains("\n\n"), "Missing proper spacing around README content");
    }
}
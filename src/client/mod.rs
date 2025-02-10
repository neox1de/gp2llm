use base64::Engine;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, AUTHORIZATION};
use anyhow::{Result, anyhow};
use crate::models::{GitHubUser, Repository, UserData};
use std::fs;

pub struct GitHubClient {
    client: reqwest::blocking::Client,
}

impl GitHubClient {
    pub fn new(token: Option<&str>) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("github-api-client"));
        
        if let Some(token) = token {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", token))
                    .map_err(|e| anyhow!("Invalid token format: {}", e))?
            );
        }
        
        let client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()?;
            
        Ok(Self { client })
    }

    pub fn fetch_user(&self, username: &str) -> Result<GitHubUser> {
        let url = format!("https://api.github.com/users/{}", username);
        let response = self.client.get(&url).send()?;
        
        if response.status() == reqwest::StatusCode::FORBIDDEN {
            let rate_limit_info = response.headers()
                .get("x-ratelimit-remaining")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("unknown");
            return Err(anyhow!("Rate limit exceeded. Remaining requests: {}", rate_limit_info));
        }

        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch user data: {}", response.status()));
        }

        let mut user: GitHubUser = response.json()?;
        
        // Fetch profile README.md if it exists
        let readme_content = self.fetch_profile_readme(username);
        user.profile_readme = readme_content.ok();
        
        Ok(user)
    }

    pub fn fetch_repositories(&self, username: &str) -> Result<Vec<Repository>> {
        let url = format!("https://api.github.com/users/{}/repos", username);
        let response = self.client.get(&url).send()?;

        if response.status() == reqwest::StatusCode::FORBIDDEN {
            let rate_limit_info = response.headers()
                .get("x-ratelimit-remaining")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("unknown");
            return Err(anyhow!("Rate limit exceeded. Remaining requests: {}", rate_limit_info));
        }

        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch repositories: {}", response.status()));
        }

        Ok(response.json()?)
    }

    fn fetch_profile_readme(&self, username: &str) -> Result<String> {
        let url = format!("https://api.github.com/repos/{}/{}/contents/README.md", username, username);
        let response = self.client.get(&url).send()?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Profile README.md not found"));
        }

        let content: serde_json::Value = response.json()?;
        let encoded_content = content["content"]
            .as_str()
            .ok_or_else(|| anyhow!("Invalid content"))?;
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(encoded_content.replace('\n', ""))?;
        
        String::from_utf8(decoded).map_err(|e| anyhow!(e))
    }

    pub fn fetch_all_data(&self, username: &str) -> Result<UserData> {
        let user = self.fetch_user(username)?;
        let repositories = self.fetch_repositories(username)?;
        let data = UserData { user, repositories };

        let json = serde_json::to_string_pretty(&data)?;
        fs::write(format!("{}.json", username), json)?; // save json output

        let md_content = self.generate_markdown(&data);
        fs::write(format!("{}.md", username), md_content)?; // save md output
        
        Ok(data)
    }

    fn generate_markdown(&self, data: &UserData) -> String {
        let user = &data.user;
        let mut md = format!(
            "# {} Profile\n\n",
            user.name.as_deref().unwrap_or(&user.login)
        );

        if let Some(bio) = &user.bio {
            md.push_str(&format!("> {}\n\n", bio));
        }

        md.push_str("## Overview\n\n");
        md.push_str(&format!("- **Username:** {}\n", user.login));
        md.push_str(&format!("- **Public Repositories:** {}\n", user.public_repos));
        md.push_str(&format!("- **Followers:** {}\n", user.followers));
        md.push_str(&format!("- **Following:** {}\n", user.following));

        if let Some(location) = &user.location {
            md.push_str(&format!("- **Location:** {}\n", location));
        }

        if let Some(company) = &user.company {
            md.push_str(&format!("- **Company:** {}\n", company));
        }

        if let Some(readme) = &user.profile_readme {
            md.push_str("\n## Profile README\n\n");
            // Just add the README content directly without code fences
            md.push_str(&format!("{}\n\n", readme));
        }

        md.push_str("## Repositories\n\n");
        for repo in &data.repositories {
            md.push_str(&format!("### {}\n", repo.name));
            if let Some(desc) = &repo.description {
                md.push_str(&format!("> {}\n\n", desc));
            }
            md.push_str(&format!("- **Stars:** {}\n", repo.stargazers_count));
            md.push_str(&format!("- **Forks:** {}\n", repo.forks_count));
            if let Some(lang) = &repo.language {
                md.push_str(&format!("- **Language:** {}\n", lang));
            }
            md.push_str(&format!("- **Created:** {}\n", repo.created_at));
            md.push_str(&format!("- **Last Updated:** {}\n\n", repo.updated_at));
        }

        md
    }
}
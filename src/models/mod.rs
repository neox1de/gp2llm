use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubUser {
    pub login: String,
    pub id: i64,
    pub avatar_url: String,
    pub html_url: String,
    pub name: Option<String>,
    pub company: Option<String>,
    pub blog: Option<String>,
    pub location: Option<String>,
    pub email: Option<String>,
    pub bio: Option<String>,
    pub public_repos: i32,
    pub followers: i32,
    pub following: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_readme: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    pub id: i64,
    pub name: String,
    pub full_name: String,
    pub private: bool,
    pub html_url: String,
    pub description: Option<String>,
    pub fork: bool,
    pub created_at: String,
    pub updated_at: String,
    pub pushed_at: String,
    pub language: Option<String>,
    pub stargazers_count: i32,
    pub watchers_count: i32,
    pub forks_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme_content: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserData {
    pub user: GitHubUser,
    pub repositories: Vec<Repository>,
}
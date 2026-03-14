use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct User {
    pub login: String,
    pub bio: Option<String>,
    pub public_repos: u32,
    pub followers: u32,
}

pub async fn get_user(token: &str) -> Result<User, reqwest::Error> {
    let client = reqwest::Client::new();

    let response = client.get("https://api.github.com/user")
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "github-tui")
        .send()
        .await?
        .json::<User>()
        .await?;

    Ok(response)
}

#[derive(Deserialize, Clone)]
pub struct Repo {
    pub name: String,
    pub description: Option<String>,
    pub stargazers_count: u32,
    pub language: Option<String>,
}

pub async fn get_repos(token: &str) -> Result<Vec<Repo>, reqwest::Error> {
    let client = reqwest::Client::new();

    let response = client.get("https://api.github.com/user/repos?per_page=100&sort=updated")
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "github-tui")
        .send()
        .await?
        .json::<Vec<Repo>>()
        .await?;

    Ok(response)
}

#[derive(Deserialize, Clone)]
pub struct RepoContent {
    pub name: String,
    #[serde(rename = "type")]
    pub content_type: String,
    pub path: String,
}

pub async fn get_repos_content(token: &str, owner: &str, repo: &str, path: &str) -> Result<Vec<RepoContent>, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!("https://api.github.com/repos/{}/{}/contents/{}", owner, repo, path);

    let response = client.get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "github-tui")
        .send()
        .await?
        .json::<Vec<RepoContent>>()
        .await?;

    Ok(response)
}

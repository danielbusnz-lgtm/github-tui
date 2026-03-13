use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    login: String,
    bio: Option<String>,
    public_repos: u32,
    followers: u32,
}

async fn get_user(token: &str) -> Result<User, reqwest::Error> {
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



#[derive(Deserialize)]
struct Repo{
    name: String,
    description: Option<String>,
    stargazers_count: u32,
    language:Option<String>,
}

async fn get_repos(token: &str) -> Result<Vec<Repo>, reqwest::Error>{
    let client = reqwest::Client::new();

    let response = client.get("https://api.github.com/user/repos?per_page=100&sort=updated")
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent","github-tui")
        .send()
        .await?
        .json::<Vec<Repo>>()
        .await?;

    Ok(response)
}

#[derive(Deserialize)]
struct RepoContent{
    name: String,
    #[serde(rename = "type")]  
    content_type: String,
    path: String,
   }
async fn get_repos_content(token: &str, owner: &str, repo: &str, path: &str) -> Result<Vec<RepoContent>, reqwest::Error>{
    let client = reqwest::Client::new();
    let url = format!("https://api.github.com/repos/{}/{}/contents/{}", owner, repo, path);

    let response = client.get(url)
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent","github-tui")
        .send()
        .await?
        .json::<Vec<RepoContent>>()
        .await?;

    Ok(response)
}

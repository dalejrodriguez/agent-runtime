use anyhow::{Result, anyhow};
use reqwest::header::AUTHORIZATION;
use reqwest::header::{HeaderMap, HeaderValue};
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        ["idle"] => loop {
            std::thread::park()
        },
        ["get", repo] if valid(repo) => match get(repo).await {
            Ok(body) => {
                print!("{body}");
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("status: {e}");
                ExitCode::from(1)
            }
        },
        _ => {
            eprintln!("usage: get <repo>");
            ExitCode::from(2)
        }
    }
}

fn valid(repo: &str) -> bool {
    !repo.is_empty() && repo.bytes().all(|b| b.is_ascii_alphanumeric())
}

async fn get(repo: &str) -> Result<String> {
    let token = std::fs::read_to_string("/run/secrets/token")?;
    let url = format!("https://raw.githubusercontent.com/edera-dev/{repo}/main/README.md");
    let token = format!("Bearer {}", token.trim());

    let mut headers = HeaderMap::new();
    let mut auth = HeaderValue::from_str(&token)?;
    auth.set_sensitive(true);
    headers.insert(AUTHORIZATION, auth);

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| anyhow!("{e:?}"))?;

    client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await
        .map_err(|e| anyhow!(e))
}

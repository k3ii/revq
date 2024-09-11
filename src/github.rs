use anyhow::{Context, Result};
use octocrab::Octocrab;
use serde_json::Value;

pub async fn fetch_prs(octocrab: &Octocrab, query: &str) -> Result<Value> {
    let response = octocrab
        .graphql::<Value>(&serde_json::json!({"query": query}))
        .await
        .context("Failed to fetch PRs from GitHub")?;

    Ok(response)
}

use octocrab::Octocrab;
use serde_json::Value;

pub async fn fetch_prs(octocrab: &Octocrab, query: &str) -> Result<Value, octocrab::Error> {
    let response = octocrab
        .graphql::<Value>(&serde_json::json!({"query": query}))
        .await?;
    Ok(response)
}

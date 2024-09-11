use anyhow::{Context, Result};
use serde_json::Value;
use skim::prelude::*;
use std::io::Cursor;

#[derive(Clone)]
pub struct PR {
    pub title: String,
    pub repo: String,
    pub url: String,
}

pub fn build_pr_list(prs: &Value) -> Result<Vec<PR>> {
    let edges = prs["data"]["search"]["edges"]
        .as_array()
        .context("Failed to parse 'edges' as array")?;

    let pr_list = edges
        .iter()
        .map(|pr| {
            let title = pr["node"]["title"]
                .as_str()
                .unwrap_or("No title")
                .to_string();
            let repo = pr["node"]["repository"]["nameWithOwner"]
                .as_str()
                .unwrap_or("Unknown repo")
                .to_string();
            let url = pr["node"]["url"].as_str().unwrap_or("").to_string();

            Ok(PR { title, repo, url })
        })
        .collect::<Result<Vec<PR>>>()?;

    Ok(pr_list)
}

pub fn select_pr(pr_list: Vec<PR>) -> Option<Vec<PR>> {
    let options = SkimOptionsBuilder::default().multi(true).build().unwrap();

    let input = pr_list
        .iter()
        .map(|pr| format!("{} - {}", pr.repo, pr.title))
        .collect::<Vec<_>>()
        .join("\n");

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));

    if let Some(out) = Skim::run_with(&options, Some(items)) {
        match out.final_key {
            Key::Enter => {
                let selected_prs: Vec<PR> = out
                    .selected_items
                    .iter()
                    .filter_map(|selected_item| {
                        let selected_output = selected_item.output().to_string();
                        pr_list
                            .iter()
                            .find(|pr| format!("{} - {}", pr.repo, pr.title) == selected_output)
                            .cloned()
                    })
                    .collect();

                return Some(selected_prs);
            }
            Key::ESC | Key::Ctrl('c') | Key::Ctrl('d') => {
                return None;
            }
            _ => {
                return None;
            }
        }
    }

    None
}

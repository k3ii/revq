pub fn build_query(username: Option<&str>, config: &crate::config::Config) -> String {
    match username {
        Some(user) => format!(
            r#"
        {{
            search(query: "is:pr is:open author:{user}", type: ISSUE, first 100) {{
                edges {{
                    node {{
                    ... on PullRequest {{
                      title
                      url
                      repository {{
                        nameWithOwner
                      }}
                    }}
                }}
                }}
            }}
        }}
        "#,
            user = user
        ),
        None => format!(
            r#"
        {{
            search(query: "is:pr is:open author:@me user:{username}", type: ISSUE, first 100) {{
                edges {{
                    node {{
                    ... on PullRequest {{
                      title
                      url
                      repository {{
                        nameWithOwner
                      }}
                    }}
                }}
                }}
            }}
        }}
        "#,
            username = config.username
        ),
    }
}

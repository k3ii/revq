pub fn build_query(
    username: Option<&str>,
    config: &crate::config::Config,
    use_org: bool,
    use_req: bool,
) -> String {
    if use_org {
        match &config.organization_settings.organization {
            Some(org) => {
                // Check if the username is supplied along with the org flag
                if let Some(user) = username {
                    let query_with_user_and_org = format!(
                        r#"
                        {{
                            search(query: "is:pr is:open author:{user} user:{org} archived:false", type: ISSUE, first: 100) {{
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
                        user = user,
                        org = org
                    );

                    if use_req {
                        format!(
                            r#"
                            {{
                                search(query: "is:pr is:open review-requested:{user} user:{org} archived:false", type: ISSUE, first: 100) {{
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
                            user = user,
                            org = org
                        )
                    } else {
                        query_with_user_and_org
                    }
                } else {
                    // If username isn't supplied, use only org-related query
                    let base_query = format!(
                        r#"
                        {{
                            search(query: "is:pr is:open author:@me user:{org} archived:false", type: ISSUE, first: 100) {{
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
                        org = org
                    );

                    if use_req {
                        format!(
                            r#"
                            {{
                                search(query: "is:pr is:open review-requested:@me user:{org} archived:false", type: ISSUE, first: 100) {{
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
                            org = org
                        )
                    } else {
                        base_query
                    }
                }
            }
            None => "Organization not specified in config".to_string(),
        }
    } else {
        match username {
            Some(user) => {
                let base_query = format!(
                    r#"
                    {{
                        search(query: "is:pr is:open author:{user} archived:false", type: ISSUE, first: 100) {{
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
                );

                if use_req {
                    format!(
                        r#"
                        {{
                            search(query: "is:pr is:open review-requested:@me author:{user} archived:false", type: ISSUE, first: 100) {{
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
                    )
                } else {
                    base_query
                }
            }
            None => {
                let base_query = format!(
                    r#"
                    {{
                        search(query: "is:pr is:open author:{username} user:{username} archived:false", type: ISSUE, first: 100) {{
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
                );

                base_query
            }
        }
    }
}

# revq
`revq` is a command-line tool to review, search and filter Github pull requests.

## Features
- List pull requests
- Filter pull requests by author
- Filter pull requests by organization
- List pull requests where reviewer is requested

## Usage

Basic usage:
```bash
revq
```
With options: `revq [OPTIONS]`

Options:
* `-u, --username <USERNAME>`  GitHub username
* `-o, --org`                  Use organization PRs
* `-r, --req`                  Show PRs where review is requested
* `-h, --help`                 Print help
* `-V, --version`              Print version

## Getting Started
### Installation
Install `revq` using cargo:
```bash
cargo install revq
```
Use brew:
```bash
brew tap k3ii/tap/revq
```

### Configuration
1. Generate a Github token with repository or content scope.

2. Initalialize `revq` with your Github token:
```bash
revq init
```



## Motivation
`revq` was created to help developers review pull requests in a more efficient way. 

Advantage of `revq` over `gh pr`:
- `revq` can be run from anywhere in your terminal whereas `gh pr` requires you to be in the repository directory.
- `revq` allows you to filter pull requests based on author and organization.
- `revq` is written in Rust.

## Future Work
[] - Allows filter by mentions
[] - Allows filter by assignees
[] - Multiple organizations

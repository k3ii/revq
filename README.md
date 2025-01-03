# revq

`revq` is a command-line tool designed to streamline the process of reviewing, searching, and filtering GitHub pull requests.

![demo](./assets/demo.gif)

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
- [Motivation](#motivation)
- [Future Work](#future-work)
- [Contributing](#contributing)
- [License](#license)

## Features

- List pull requests with ease
- Filter pull requests by author
- Filter pull requests by organization
- List pull requests where a review is requested
- Select and open multiple pull requests simultaneously in your default browser
- User-friendly command-line interface

## Installation

You can install `revq` using one of the following methods:

### Using Cargo

If you have Rust's package manager, Cargo, installed, you can install `revq` with:

```bash
cargo install revq
```

### Using Homebrew

For macOS users, you can install `revq` using Homebrew:

```bash
brew install k3ii/tap/revq
```
Check the release page to install the pre-built binaries.

#### Arch Linux

On Arch Linux, you can install `revq` using the [AUR package](https://aur.archlinux.org/packages/revq):

```bash
paru -S revq
```

## Usage

The general syntax for using `revq` is:

```bash
revq [OPTIONS] [COMMAND]
```

### Options and Subcommands

- `-u, --username <USERNAME>`  Specify GitHub username
- `-o, --org`                  Use organization PRs
- `-r, --req`                  Show PRs where review is requested
- `    --all`                  Show all PRs for organization (only works with --org)
- `-h, --help`                 Print help information
- `-V, --version`              Print version information

### Examples

List your pull requests:
```bash
revq
```

List pull requests for a specific user:
```bash
revq -u username
```

List your pull requests for your organization:
```bash
revq -o
```

List pull requests for a specific user in your organization:
```bash
revq -o -u username
```

List pull requests where your review is requested:
```bash
revq -r
```

List pull requests where your review is requested in your organization:
```bash
revq -r -o
```

List all pull requests for your organization:
```bash
revq -o --all
```

## Configuration

To get started with `revq`, follow these steps:

1. Generate a GitHub Personal access tokens (Fine-grained token) with Content permission having Read-only access.
2. Initialize `revq` with your GitHub token:

```bash
revq init
```

3. Follow the prompts to enter your GitHub username, token and organization optionally.

To reinitialize or update your configuration:

```bash
revq init --force
```

## Motivation

`revq` was born out of a desire to learn Rust programming and improve upon a simple bash script created by my [mentor](https://github.com/puzzledvacuum).

`revq` offers several advantages over similar tools like `gh pr`:
- Can be run from any directory in your terminal
- Allows filtering of pull requests based on author and organization
- Enables selection and opening of multiple PRs simultaneously
- Written in Rust for performance and reliability

## Future Work

We have exciting plans for future enhancements to `revq`:

- [ ] Allow filtering by mentions
- [ ] Allow filtering by assignees
- [ ] Support for multiple organizations

## Contributing

Contributions are welcome to `revq`! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

If you encounter any problems or have any questions, please [open an issue](https://github.com/k3ii/revq/issues) on our GitHub repository.

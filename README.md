# GP2LLM - github Profile to a single file

A Rust CLI tool that fetches GitHub user profiles and their repositories, generates a json and a markdown file for using with Large Language Models

## Features

- Fetch comprehensive GitHub user profile information
- Retrieve user's repositories with detailed statistics
- Automatically detect and include profile README.md content
- Generate both JSON and Markdown outputs
- Handle GitHub API rate limiting
- Support for authenticated requests via GitHub token

## Prerequisites

- Rust (cargo)
- GitHub Personal Access Token (optional, but recommended to avoid rate limiting)

## Installation

You can install GP2LLM using one of these methods:

### Using Cargo Install

```bash
cargo install --git https://github.com/neox1de/gp2llm
```

### Building from Source

Clone the repository and build using Cargo:

```bash
git clone https://github.com/neox1de/gp2llm.git
cd gp2llm
cargo build --release
```

The compiled binary will be available at `target/release/gp2llm`

## Usage

### Basic Usage

```bash
gp2llm <github_username>
```

### With GitHub Token (Recommended)

Set your GitHub token as an environment variable:

```bash
export GITHUB_TOKEN=your_github_token_here
gp2llm <github_username>
```

The tool will generate two files in your current directory:
- `<username>.json`: Contains raw data in JSON format
- `<username>.md`: A formatted Markdown representation of the profile

## Output Format

### JSON Output
The JSON output includes:
- User profile information
- List of repositories
- Profile README content (if available)
- Repository statistics and metadata

### Markdown Output
The Markdown file includes:
- User name and bio
- Profile overview (followers, following, etc.)
- Profile README content (if available)
- Detailed repository information
  - Description
  - Stars and forks
  - Programming language
  - Creation and update dates

## Dependencies

- `anyhow`: Error handling
- `base64`: README content decoding
- `reqwest`: HTTP client for GitHub API
- `serde`: JSON serialization/deserialization
- `tokio`: Async runtime support
- `serde_json`: JSON processing


## Testing

Run the test suite with:

```bash
cargo test
```

The test suite includes:
- Multiple user fetching tests
- Invalid user handling
- Content formatting validation


## Contributing

Feel free to open issues or submit pull requests.


## License

This project is licensed under the MIT License - see the LICENSE file for details.
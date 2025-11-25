## RSS CLI
A simple command-line RSS feed reader written in Rust.

# Installation
cargo build --release


## Usage

# Add a feed
cargo run -- add --url "https://news.ycombinator.com/rss"

# Fetch and display feed

# Fetch 5 items (default)
cargo run -- fetch --url "https://news.ycombinator.com/rss"

# Fetch 10 items
cargo run -- fetch --url "https://blog.rust-lang.org/feed.xml" --limit 10


# Read a specific item
cargo run -- read --id 1

# Example Feeds
- Hacker News: https://news.ycombinator.com/rss
- Rust Blog: https://blog.rust-lang.org/feed.xml
- BBC News: http://feeds.bbci.co.uk/news/rss.xml
- TechCrunch: https://techcrunch.com/feed/

# Dependencies
[dependencies]
clap = { version = "4.0", features = ["derive"] }
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1", features = ["full"] }
rss = "2.0"
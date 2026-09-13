pub mod github;
pub mod local;

pub use github::{ingest_github, parse_git_url};
pub use local::ingest_local;

pub mod ignore_rules;
pub mod scan;

pub use scan::{build_file_tree, detect_language, scan_directory};

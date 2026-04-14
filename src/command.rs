use crate::language::Language;
use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    /// Language to target
    #[arg(default_value_t = Language::All)]
    pub language: Language,
    /// Caution! If set it will wipe all folders found! Unset by default
    #[arg(short, long)]
    pub wipe: bool,
    /// Absolute paths to ignore
    #[arg(short, long, value_parser)]
    pub ignores: Vec<PathBuf>,
    /// Path to search
    #[arg(short, long, default_value = ".")]
    pub path: PathBuf,
}

#[cfg(test)]
mod tests {
    use std::io;

    use rstest::rstest;

    use crate::command::Language;

    #[rstest]
    #[case("node", Language::Node)]
    #[case("rust", Language::Rust)]
    #[case("terraform", Language::Terraform)]
    #[case("RUST", Language::Rust)]
    #[case("ruSt ", Language::Rust)]
    #[case("all", Language::All)]
    fn language_string_to_enum(#[case] language_string: &str, #[case] language_enum: Language) {
        assert_eq!(language_string.parse().ok(), Some(language_enum));
    }

    #[rstest]
    #[case("node-modules")]
    #[case("rustt")]
    fn language_string_to_enum_error(#[case] language_string: &str) {
        let result = language_string.parse::<Language>();
        let err = result.err().unwrap();

        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
        assert_eq!(
            err.to_string(),
            "Valid options are: node | rust | terraform"
        );
    }

    #[rstest]
    #[case(Language::Node, "node")]
    #[case(Language::Rust, "rust")]
    #[case(Language::Terraform, "terraform")]
    #[case(Language::Python, "python")]
    #[case(Language::Zig, "zig")]
    fn language_enum_to_string(#[case] language_enum: Language, #[case] language_string: &str) {
        assert_eq!(language_enum.to_string(), language_string);
    }

    #[rstest]
    #[case(Language::Node, &["node_modules"])]
    #[case(Language::Rust, &["target"])]
    #[case(Language::Terraform, &[".terraform"])]
    #[case(Language::Python, &["__pycache__", ".venv", "venv"])]
    #[case(Language::Zig, &[".zig-cache", "zig-out"])]
    fn language_enum_dir(#[case] directory_enum: Language, #[case] directories: &[&str]) {
        assert!(
            directory_enum
                .dirs()
                .iter()
                .all(|dir| directories.contains(dir))
        );
    }
}

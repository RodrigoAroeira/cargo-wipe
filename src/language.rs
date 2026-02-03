use clap::ValueEnum;
use std::{fmt, io, str::FromStr};
use strum::EnumIter;

#[derive(Debug, PartialEq, Eq, Copy, Clone, ValueEnum, EnumIter)]
pub enum Language {
    All,
    Node,
    Rust,
    Terraform,
    Python,
    Zig,
}

impl Language {
    /// Returns a slice with directories associated with the language
    pub const fn dirs(&self) -> &[&str] {
        match self {
            Language::All => panic!("Language::All should never call dirs"),
            Language::Node => &["node_modules"],
            Language::Rust => &["target"],
            Language::Terraform => &[".terraform"],
            Language::Python => &[".venv", "venv", "__pycache__"],
            Language::Zig => &[".zig-cache", "zig-out"],
        }
    }
}

impl FromStr for Language {
    type Err = io::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().trim() {
            "all" => Ok(Language::All),
            "node" => Ok(Language::Node),
            "rust" => Ok(Language::Rust),
            "terraform" => Ok(Language::Terraform),
            "python" => Ok(Language::Python),
            "zig" => Ok(Language::Zig),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Valid options are: node | rust | terraform",
            )),
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Language::All => write!(f, "all"),
            Language::Node => write!(f, "node"),
            Language::Rust => write!(f, "rust"),
            Language::Terraform => write!(f, "terraform"),
            Language::Python => write!(f, "python"),
            Language::Zig => write!(f, "zig"),
        }
    }
}

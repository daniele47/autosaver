use anyhow::{Result, bail};
use tracing::{instrument, warn};

use crate::prof::Profile;

#[derive(Debug, Clone, PartialEq, Eq)]
enum RawProfileLine<'a> {
    Option(&'a str),
    Data(&'a str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RawProfile<'a> {
    lines: Vec<RawProfileLine<'a>>,
    kind: &'a str,
}

impl<'a> RawProfile<'a> {
    #[instrument(ret, level = "trace")]
    pub fn parse_config(config: &'a str) -> Self {
        let mut lines = Vec::new();
        let mut kind = "";

        for line in config.lines() {
            if let Some(option) = line.strip_prefix("/!") {
                let option = option.trim();
                if let Some(kind_str) = option.strip_prefix("kind") {
                    let kind_str = kind_str.trim();
                    kind = kind_str;
                } else {
                    lines.push(RawProfileLine::Option(option));
                }
            } else if !line.starts_with("/") {
                if !line.starts_with("//") {
                    warn!("Comments are meant to start with // ({line})");
                }
                lines.push(RawProfileLine::Data(line.trim()));
            }
        }

        Self { lines, kind }
    }
}

impl Profile {
    pub fn parse_profile(config: &str) -> Result<Profile> {
        let raw = RawProfile::parse_config(config);
        match raw.kind {
            "" | "composite" => {}
            "module" => {}
            "runner" => {}
            _ => bail!("Invalid kind option: {}", raw.kind),
        }
        todo!()
    }
}

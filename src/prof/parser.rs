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
    name: &'a str,
    id: &'a str,
}

impl<'a> RawProfile<'a> {
    #[instrument(ret, level = "trace")]
    pub fn parse_config(config: &'a str, name: &'a str) -> Self {
        let mut lines = Vec::new();
        let mut kind = "";
        let mut id = name;

        for line in config.lines() {
            // option lines
            if let Some(option) = line.strip_prefix("/!") {
                let option = option.trim();
                // specific shared options
                if let Some(kind_str) = option.strip_prefix("kind") {
                    let kind_str = kind_str.trim();
                    kind = kind_str;
                } else if let Some(id_str) = option.strip_prefix("id") {
                    let id_str = id_str.trim();
                    id = id_str;
                }
                // fallback to storing not shared options
                else {
                    lines.push(RawProfileLine::Option(option));
                }
            }
            // data lines
            else if !line.starts_with("/") {
                if !line.starts_with("//") {
                    warn!("Comments are meant to start with // ({line})");
                }
                lines.push(RawProfileLine::Data(line.trim()));
            }
        }

        Self {
            lines,
            kind,
            name,
            id,
        }
    }
}

impl Profile {
    pub fn parse_profile(config: String, name: String) -> Result<Profile> {
        let raw = RawProfile::parse_config(&config, &name);
        match raw.kind {
            "" | "composite" => {}
            "module" => {}
            "runner" => {}
            _ => bail!("Invalid kind option: {}", raw.kind),
        }
        todo!()
    }
}

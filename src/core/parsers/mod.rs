//! This module has utilities to parse all kind of profile configuration files.
//!
//! Parsing happens in 2 stages:
//! - parsing into raw state (basic intermidiate state, shared between all config parsers)
//! - parsing into the actual config

use crate::core::{
    error::{Error, Result},
    fs::LineReader,
    parsers::{composite::CompositeParser, module::ModuleParser},
    profile::Profile,
};

mod composite;
mod module;

#[derive(Debug)]
enum RawKind {
    Option,
    Data,
}

#[derive(Debug)]
struct RawItem {
    line: usize,
    content: String,
    kind: RawKind,
}

#[derive(Debug)]
struct RawParser {}

impl Profile {
    /// Parse config file line by line into proper struct.
    pub fn parse(profile: String, reader: impl LineReader) -> Result<Self> {
        let mut raw = RawParser::parse(reader);
        if let Some(first) = raw.next() {
            let first = first?;
            let content = first.content;

            // profile line MUST be the very first
            if first.line != 1 {
                return Err(Error::InvalidOptionLine(profile, 1, content));
            }

            // pick correct parser based on the profile type parsed from the first line
            match content.as_str() {
                "type profile" => CompositeParser::parse(profile, raw),
                "type module" => ModuleParser::parse(profile, raw),
                _ => {
                    return Err(Error::InvalidOptionLine(profile, 1, content));
                }
            }
        } else {
            return Err(Error::MissingProfileType(profile));
        }
    }
}

impl RawParser {
    fn parse_line(line: (usize, Result<String>)) -> Result<Option<RawItem>> {
        let str = line.1?;
        let line = line.0 + 1;
        let content;
        let kind;

        // option line
        if str.starts_with("/!") {
            kind = RawKind::Option;
            content = str[2..].trim().to_string();
        }
        // comment line
        else if str.starts_with("/") {
            return Ok(None);
        }
        // data line
        else {
            kind = RawKind::Data;
            content = str.trim().to_string();
        }

        // remove empty lines, or lines that had only empty lines
        if content.is_empty() {
            return Ok(None);
        }

        Ok(Some(RawItem {
            line,
            content,
            kind,
        }))
    }

    fn parse(reader: impl LineReader) -> impl Iterator<Item = Result<RawItem>> {
        reader
            .into_iter()
            .enumerate()
            .filter_map(|i| Self::parse_line(i).transpose())
    }
}

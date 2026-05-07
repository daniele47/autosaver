//! This module has utilities to parse all kind of profile configuration files.
//!
//! Parsing happens in 2 stages:
//! - parsing into raw state (basic intermidiate state, shared between all config parsers)
//! - parsing into the actual config

use crate::core::{
    error::{Error, Result},
    fs::LineReader,
    parsers::{composite::CompositeParser, module::ModuleParser, runner::RunnerParser},
    profile::Profile,
};

mod composite;
mod module;
mod runner;

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
                return Err(Error::InvalidOptionLine(
                    profile,
                    first.line,
                    content,
                    "".into(),
                ));
            }

            // pick correct parser based on the profile type parsed from the first line
            match content.as_str() {
                "type composite" => CompositeParser::parse(profile, raw),
                "type module" => ModuleParser::parse(profile, raw),
                "type runner" => RunnerParser::parse(profile, raw),
                _ => Err(Error::InvalidOptionLine(
                    profile,
                    first.line,
                    content,
                    "".into(),
                )),
            }
        } else {
            Err(Error::MissingProfileType(profile))
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
        if let Some(stripped) = str.strip_prefix("/!") {
            kind = RawKind::Option;
            content = stripped.trim().to_string();
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

#[cfg(test)]
mod tests {
    use crate::core::fs::AnyLineReader;

    use super::*;

    #[test]
    fn test_raw_parser() -> Result<()> {
        let lines = [
            Ok("/! type module".to_string()),
            Ok("/ this is a comment".to_string()),
            Ok("src/lib.rs".to_string()),
            Ok("   /! policy track   ".to_string()),
            Ok("   ".to_string()),
            Ok("target/".to_string()),
        ];
        let reader = AnyLineReader::new(lines.into_iter());

        let items: Vec<RawItem> = RawParser::parse(reader).collect::<Result<Vec<_>>>()?;

        print!("{items:#?}");
        assert_eq!(items.len(), 4);

        // First: option line
        assert_eq!(items[0].line, 1);
        assert_eq!(items[0].content, "type module");
        assert!(matches!(items[0].kind, RawKind::Option));

        // Second: data line (comment skipped)
        assert_eq!(items[1].line, 3);
        assert_eq!(items[1].content, "src/lib.rs");
        assert!(matches!(items[1].kind, RawKind::Data));

        // Third: option line with whitespace trimmed
        assert_eq!(items[2].line, 4);
        assert_eq!(items[2].content, "/! policy track");
        assert!(matches!(items[2].kind, RawKind::Data));

        // Fourth: data line (empty line skipped)
        assert_eq!(items[3].line, 6);
        assert_eq!(items[3].content, "target/");
        assert!(matches!(items[3].kind, RawKind::Data));

        Ok(())
    }
}

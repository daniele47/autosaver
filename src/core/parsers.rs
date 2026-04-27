//! This module has utilities to parse all kind of profile configuration files.

use crate::core::{errors::Result, fs::LineReader, module::Module, profile::Profile};

/// All possible kind of parsed configs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsedConfig {
    Profile(Profile),
    Module(Module),
}

// represent intermidiate parsing step
enum RawKind {
    Option,
    Data,
}

struct RawItem {
    line: usize,
    content: String,
    kind: RawKind,
}

fn parse_line(line: (usize, Result<String>)) -> Result<Option<RawItem>> {
    let str = line.1?;
    let line = line.0;
    let content;
    let kind;

    // option line
    if str.starts_with("/!") {
        kind = RawKind::Option;
        content = str[2..].trim().to_string();
    }
    // comment line
    else if str.is_empty() || str.starts_with("/") {
        return Ok(None);
    }
    // data line
    else {
        kind = RawKind::Data;
        content = str[2..].trim().to_string();
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
        .filter_map(|i| parse_line(i).transpose())
}

// TO BE REMOVED ONCE ALL IS IMPLEMENTED:
//
// Have 3 types of parsers:
// - raw parser (takes iterator of lines and map them to an intermidiate config struct)
// - a parser x each config type (that actually takes the raw parser and correctly creates its proper config type)
// - wrapper parser (handles EVERYTHING. from raw config iterator returns a `ParsedConfig`)
//
// NOTE: THIS ENTIRE PARSING SYSTEM WILL NEVER FULLY ALLOCATE EVERYTHING INTO MEMORY. Just have
// iterator everywhere, until a final ParsedConfig is achieved:
// Iterator<file_lines> -> Iterator<raw_config> -> Fully loaded `ParsedConfig`

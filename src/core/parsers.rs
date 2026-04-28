//! This module has utilities to parse all kind of profile configuration files.
//!
//! Parsing happens in 2 stages:
//! - parsing into raw state (basic intermidiate state, shared between all config parsers)
//! - parsing into the actual config

use crate::core::{
    error::{Error, Result},
    fs::LineReader,
    profile::{Profile, ProfileType, module::Module},
};

/// All possible kind of parsed configs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigParser {
    Profile(Profile),
    Module(Module),
}

// represent intermidiate parsing step
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

#[derive(Debug)]
struct ModuleParser {}

#[derive(Debug)]
struct ProfileParser {}

impl ConfigParser {
    /// Parse config file line by line into proper struct.
    pub fn parse(profile: String, reader: impl LineReader) -> Result<Self> {
        let mut raw = RawParser::parse(reader);
        if let Some(first) = raw.next() {
            let first = first?;
            let content = first.content;

            // profile line MUST be the very first
            if first.line != 1 {
                return Err(Error::InvalidOptionLine {
                    name: profile,
                    line: (1, content),
                });
            }

            // pick correct parser based on the profile type parsed from the first line
            match content.as_str() {
                "type profile" => ProfileParser::parse(profile, raw).map(ConfigParser::Profile),
                "type module" => ModuleParser::parse(profile, raw).map(ConfigParser::Module),
                _ => {
                    return Err(Error::InvalidOptionLine {
                        name: profile,
                        line: (1, content),
                    });
                }
            }
        } else {
            return Err(Error::MissingProfileType { name: profile });
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

impl ModuleParser {
    fn parse(profile: String, raw: impl Iterator<Item = Result<RawItem>>) -> Result<Module> {
        todo!()
    }
}

impl ProfileParser {
    fn parse(profile: String, raw: impl Iterator<Item = Result<RawItem>>) -> Result<Profile> {
        let mut entries = vec![];

        for line in raw {
            let line = line?;
            match line.kind {
                // composite profile has NO options lines
                RawKind::Option => {
                    return Err(Error::InvalidOptionLine {
                        name: profile,
                        line: (line.line, line.content),
                    });
                }

                // normal data lines, aka profile names here
                RawKind::Data => {
                    if !line
                        .content
                        .chars()
                        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
                    {
                        // limit valid profile names!
                        return Err(Error::InvalidDataLine {
                            name: profile,
                            line: (line.line, line.content),
                        });
                    }
                    entries.push(line.content);
                }
            }
        }

        Ok(Profile::new(profile, entries, ProfileType::Composite))
    }
}

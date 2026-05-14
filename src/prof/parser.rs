use std::str::FromStr;

use anyhow::{Result, bail};
use tracing::{instrument, warn};

use crate::{
    fs::rel::RelPathStr,
    prof::{
        Profile, ProfileKind,
        composite::{Composite, CompositeEntry},
        module::Module,
        runner::Runner,
    },
};

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
    pub fn parse_config(config: &'a str, name: &'a str) -> Result<Self> {
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
            // comment lines
            else if line.starts_with("/") {
                if !line.starts_with("//") {
                    warn!("Comments are meant to start with double slashes: {line}");
                }
            }
            // data lines
            else {
                let line = line.trim();
                if !line.is_empty() {
                    lines.push(RawProfileLine::Data(line));
                }
            }
        }

        if kind.is_empty() {
            bail!("Option 'kind' is missing from profile {name}");
        } else if id.is_empty() {
            bail!("Option 'id' is missing from profile {name}");
        }

        Ok(Self {
            lines,
            kind,
            name,
            id,
        })
    }
}

impl Profile {
    pub fn parse_profile(config: String, name: String) -> Result<Profile> {
        let raw = RawProfile::parse_config(&config, &name)?;
        match raw.kind {
            "composite" => Self::parse_composite(raw),
            "module" => Self::parse_module(raw),
            "runner" => Self::parse_runner(raw),
            _ => bail!(
                "Unknown kind option for profile {}: '{}'. Only valid kinds are 'composite', 'module' and 'runner'",
                raw.name,
                raw.kind,
            ),
        }
    }

    fn parse_composite(raw: RawProfile) -> Result<Self> {
        let mut entries = vec![];
        for line in raw.lines {
            match line {
                RawProfileLine::Option(opt) => {
                    bail!("Invalid option '{opt}' for composite profile {}", raw.name);
                }
                RawProfileLine::Data(data) => {
                    let entry = CompositeEntry::new(RelPathStr::from_str(data)?);
                    entries.push(entry);
                }
            }
        }
        let name = RelPathStr::from_str(raw.name)?;
        let id = RelPathStr::from_str(raw.id)?;
        let kind = ProfileKind::Composite(Composite::new(entries));
        Ok(Profile::new(name, id, kind))
    }

    fn parse_module(raw: RawProfile) -> Result<Self> {
        let entries = vec![];
        let name = RelPathStr::from_str(raw.name)?;
        let id = RelPathStr::from_str(raw.id)?;
        let kind = ProfileKind::Module(Module::new(entries));
        Ok(Profile::new(name, id, kind))
    }

    fn parse_runner(raw: RawProfile) -> Result<Self> {
        let entries = vec![];
        let name = RelPathStr::from_str(raw.name)?;
        let id = RelPathStr::from_str(raw.id)?;
        let kind = ProfileKind::Runner(Runner::new(entries));
        Ok(Profile::new(name, id, kind))
    }
}

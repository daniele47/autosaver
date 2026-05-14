use std::str::FromStr;

use anyhow::{Context, Error, Result, anyhow, bail};
use tracing::{instrument, warn};

use crate::{
    fs::rel::RelPathStr,
    prof::{
        Profile, ProfileKind,
        composite::{Composite, CompositeEntry},
        module::{Module, ModuleEntry, ModulePolicy},
        runner::{Runner, RunnerEntry, RunnerPolicy},
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum RawProfileLine<'a> {
    Option(&'a str, usize),
    Data(&'a str, usize),
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

        for (i, line) in config.lines().enumerate() {
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
                    lines.push(RawProfileLine::Option(option, i));
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
                    lines.push(RawProfileLine::Data(line, i));
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
                "Unknown kind option for profile {}: '{}'",
                raw.name,
                raw.kind,
            ),
        }
    }

    fn parse_composite(raw: RawProfile) -> Result<Self> {
        let mut entries = vec![];
        let kind = "composite";
        for line in raw.lines {
            match line {
                RawProfileLine::Option(opt, i) => {
                    bail!(Self::err_opt(raw.name, opt, i, kind));
                }
                RawProfileLine::Data(data, i) => {
                    let data = Self::data_ctx(raw.name, data, i, kind)?;
                    let entry = CompositeEntry::new(data);
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
        let mut entries = vec![];
        let mut policy = ModulePolicy::Track;
        let kind = "module";
        for line in raw.lines {
            match line {
                RawProfileLine::Option(opt, i) => match opt {
                    opt_policy if let Some(opt_val) = opt_policy.strip_prefix("policy") => {
                        match opt_val.trim() {
                            "ignore" => policy = ModulePolicy::Ignore,
                            "notdiff" => policy = ModulePolicy::NotDiff,
                            "track" => policy = ModulePolicy::Track,
                            _ => bail!(Self::err_val(raw.name, opt, i, kind)),
                        }
                    }
                    _ => bail!(Self::err_opt(raw.name, opt, i, kind)),
                },
                RawProfileLine::Data(data, i) => {
                    let data = Self::data_ctx(raw.name, data, i, kind)?;
                    let entry = ModuleEntry::new(data, policy);
                    entries.push(entry);
                }
            }
        }
        let name = RelPathStr::from_str(raw.name)?;
        let id = RelPathStr::from_str(raw.id)?;
        let kind = ProfileKind::Module(Module::new(entries));
        Ok(Profile::new(name, id, kind))
    }

    fn parse_runner(raw: RawProfile) -> Result<Self> {
        let mut entries = vec![];
        let mut policy = RunnerPolicy::Run;
        let kind = "runner";
        for line in raw.lines {
            match line {
                RawProfileLine::Option(opt, i) => match opt {
                    opt_policy if let Some(opt_val) = opt_policy.strip_prefix("policy") => {
                        match opt_val.trim() {
                            "run" => policy = RunnerPolicy::Run,
                            "skip" => policy = RunnerPolicy::Skip,
                            _ => bail!(Self::err_val(raw.name, opt, i, kind)),
                        }
                    }
                    _ => bail!(Self::err_opt(raw.name, opt, i, kind)),
                },
                RawProfileLine::Data(data, i) => {
                    let data = Self::data_ctx(raw.name, data, i, kind)?;
                    let entry = RunnerEntry::new(data, policy);
                    entries.push(entry);
                }
            }
        }
        let name = RelPathStr::from_str(raw.name)?;
        let id = RelPathStr::from_str(raw.id)?;
        let kind = ProfileKind::Runner(Runner::new(entries));
        Ok(Profile::new(name, id, kind))
    }

    // packaged error messages
    fn data_ctx(name: &str, data: &str, i: usize, kind: &str) -> Result<RelPathStr> {
        RelPathStr::from_str(data)
            .with_context(|| format!("Invalid data '{data}' for {kind} profile {name} at line {i}"))
    }
    fn err_opt(name: &str, opt: &str, i: usize, kind: &str) -> Error {
        let mut opt_split = opt.split_whitespace();
        let opt1 = opt_split.next().unwrap_or("");
        anyhow!("Invalid option '{opt1}' for {kind} profile {name} at line {i}")
    }
    fn err_val(name: &str, opt: &str, i: usize, kind: &str) -> Error {
        let mut opt_split = opt.split_whitespace();
        let opt1 = opt_split.next().unwrap_or("");
        let opt2 = opt_split.next().unwrap_or("");
        anyhow!("Option '{opt1}' for {kind} profile {name} at line {i} has invalid value '{opt2}'")
    }
}

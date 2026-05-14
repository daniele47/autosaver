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
            let line = line.trim();
            if let Some(opt) = line.strip_prefix("/!").map(str::trim) {
                // specific shared options
                if let Some(kind_str) = opt.strip_prefix("kind").map(str::trim) {
                    kind = kind_str;
                } else if let Some(id_str) = opt.strip_prefix("id").map(str::trim) {
                    id = id_str;
                }
                // fallback to storing not shared options
                else {
                    lines.push(RawProfileLine::Option(opt, i));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prof::ProfileKind;

    #[test]
    fn test_parse_composite_profile() -> Result<()> {
        let config = r#"
            /! kind composite
            /! id profiles/my_composite
            path/to/file1.txt
            path/to/file2.txt
            // This is a comment
            path/to/file3.txt
        "#
        .to_string();

        let profile = Profile::parse_profile(config, "my_composite".to_string())?;

        assert_eq!(profile.name().as_ref(), "my_composite");
        assert_eq!(profile.id().as_ref(), "profiles/my_composite");

        match profile.kind() {
            ProfileKind::Composite(composite) => {
                assert_eq!(composite.entries().len(), 3);
            }
            _ => panic!("Expected Composite profile"),
        }

        Ok(())
    }

    #[test]
    fn test_parse_module_profile() -> Result<()> {
        let config = r#"
            /! kind module
            /! id profiles/my_module
            /! policy track
            src/main.rs
            src/lib.rs
            /! policy ignore
            target/
            /! policy notdiff
            Cargo.lock
        "#
        .to_string();

        let profile = Profile::parse_profile(config, "my_module".to_string())?;

        assert_eq!(profile.name().as_ref(), "my_module");
        assert_eq!(profile.id().as_ref(), "profiles/my_module");

        match profile.kind() {
            ProfileKind::Module(module) => {
                let entries = module.entries();
                assert_eq!(entries.len(), 4);

                // First two should be Track
                assert_eq!(*entries[0].policy(), ModulePolicy::Track);
                assert_eq!(*entries[1].policy(), ModulePolicy::Track);

                // Third should be Ignore
                assert_eq!(*entries[2].policy(), ModulePolicy::Ignore);

                // Fourth should be NotDiff
                assert_eq!(*entries[3].policy(), ModulePolicy::NotDiff);
            }
            _ => panic!("Expected Module profile"),
        }

        Ok(())
    }

    #[test]
    fn test_parse_runner_profile() -> Result<()> {
        let config = r#"
            /! kind runner
            /! id profiles/my_runner
            /! policy run
            echo "Hello"
            ls -la
            /! policy skip
            rm -rf /tmp/test
            /! policy run
            cargo build
        "#
        .to_string();

        let profile = Profile::parse_profile(config, "my_runner".to_string())?;

        assert_eq!(profile.name().as_ref(), "my_runner");
        assert_eq!(profile.id().as_ref(), "profiles/my_runner");

        match profile.kind() {
            ProfileKind::Runner(runner) => {
                let entries = runner.entries();
                assert_eq!(entries.len(), 4);

                assert_eq!(*entries[0].policy(), RunnerPolicy::Run);
                assert_eq!(*entries[1].policy(), RunnerPolicy::Run);
                assert_eq!(*entries[2].policy(), RunnerPolicy::Skip);
                assert_eq!(*entries[3].policy(), RunnerPolicy::Run);
            }
            _ => panic!("Expected Runner profile"),
        }

        Ok(())
    }
}

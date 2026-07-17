use std::str::FromStr;

use anyhow::{Context, anyhow, bail};
use indexmap::IndexSet;

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
    id: Option<&'a str>,
}

impl<'a> RawProfile<'a> {
    fn parse_config(config: &'a str, name: &'a str) -> anyhow::Result<Self> {
        let mut lines = Vec::new();
        let mut kind = "";
        let mut id = None;

        for (i, line) in config.lines().enumerate() {
            // option lines
            let i = i + 1;
            let line = line.trim();
            if let Some(opt) = line.strip_prefix("/!").map(str::trim) {
                // specific shared options
                if let Some(kind_str) = opt.strip_prefix("kind ").map(str::trim) {
                    if !kind.is_empty() {
                        bail!(Profile::err_dup(name, "kind", i));
                    }
                    kind = kind_str;
                } else if let Some(id_str) = opt.strip_prefix("id ").map(str::trim) {
                    if id.is_some() {
                        bail!(Profile::err_dup(name, "id", i));
                    }
                    id = Some(id_str);
                }
                // fallback to storing not shared options
                else {
                    lines.push(RawProfileLine::Option(opt, i));
                }
            }
            // comment lines
            else if line.starts_with("//") || line.is_empty() {
            }
            // invalid reserved lines
            else if line.starts_with("/") {
                let str_init: String = line.chars().take(2).collect();
                bail!(format!(
                    "Invalid start of line '{str_init}' for {kind} profile '{name}' \
                    (line {i}): currently reserved for potential future use"
                ));
            }
            // data lines
            else {
                lines.push(RawProfileLine::Data(line, i));
            }
        }

        if kind.is_empty() {
            bail!("Option 'kind' is missing from profile '{name}'");
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
    pub fn parse_config(config: &str, name: &str) -> anyhow::Result<Profile> {
        let raw = RawProfile::parse_config(config, name)?;
        match raw.kind {
            "composite" => Self::parse_composite(raw),
            "module" => Self::parse_module(raw),
            "runner" => Self::parse_runner(raw),
            _ => bail!(
                "Unknown 'kind' option for profile '{}': '{}'",
                raw.name,
                raw.kind,
            ),
        }
    }

    fn parse_composite(raw: RawProfile) -> anyhow::Result<Self> {
        let mut entries = vec![];
        let kind = "composite";
        for line in raw.lines {
            match line {
                RawProfileLine::Option(opt, i) => {
                    bail!(Self::err_opt(raw.name, opt, i, kind));
                }
                RawProfileLine::Data(data, i) => {
                    let child = Self::data_ctx(raw.name, data, i, kind)?;
                    let entry = CompositeEntry { child };
                    entries.push(entry);
                }
            }
        }
        let id = raw.id.map(RelPathStr::from_str).transpose()?;
        let kind = ProfileKind::Composite(Composite { entries });
        Ok(Profile { id, kind })
    }

    fn parse_module(raw: RawProfile) -> anyhow::Result<Self> {
        let mut entries = vec![];
        let mut cleanup = IndexSet::new();
        let mut policy = ModulePolicy::Include;
        let kind = "module";
        for line in raw.lines {
            match line {
                RawProfileLine::Option(opt, i) => match opt {
                    opt_policy if let Some(opt_val) = opt_policy.strip_prefix("policy") => {
                        match opt_val.trim() {
                            "exclude" => policy = ModulePolicy::Exclude,
                            "notdiff" => policy = ModulePolicy::NotDiff,
                            "include" => policy = ModulePolicy::Include,
                            _ => bail!(Self::err_val(raw.name, opt, i, kind)),
                        }
                    }
                    opt_cleanup if let Some(opt_val) = opt_cleanup.strip_prefix("cleanup") => {
                        let opt_val = opt_val.trim();
                        let opt_val_relpath = RelPathStr::from_str(opt_val).with_context(|| {
                            format!("{} TODO: proper error msg if opt_val invalid relpath", 2)
                        })?;
                        cleanup.insert(opt_val_relpath);
                    }
                    _ => bail!(Self::err_opt(raw.name, opt, i, kind)),
                },
                RawProfileLine::Data(data, i) => {
                    let path = Self::data_ctx(raw.name, data, i, kind)?;
                    let entry = ModuleEntry { path, policy };
                    entries.push(entry);
                }
            }
        }
        let id = raw.id.map(RelPathStr::from_str).transpose()?;
        let cleanup = cleanup.into_iter().collect();
        let kind = ProfileKind::Module(Module { entries, cleanup });
        Ok(Profile { id, kind })
    }

    fn parse_runner(raw: RawProfile) -> anyhow::Result<Self> {
        let mut entries = vec![];
        let mut policy = RunnerPolicy::Include;
        let mut stdin = false;
        let kind = "runner";
        for line in raw.lines {
            match line {
                RawProfileLine::Option(opt, i) => match opt {
                    opt_policy if let Some(opt_val) = opt_policy.strip_prefix("policy") => {
                        match opt_val.trim() {
                            "exclude" => policy = RunnerPolicy::Exclude,
                            "include" => policy = RunnerPolicy::Include,
                            _ => bail!(Self::err_val(raw.name, opt, i, kind)),
                        }
                    }
                    opt_set if let Some(opt_val) = opt_set.strip_prefix("stdin") => {
                        match opt_val.trim() {
                            "on" => stdin = true,
                            "off" => stdin = false,
                            _ => bail!(Self::err_val(raw.name, opt, i, kind)),
                        }
                    }
                    _ => bail!(Self::err_opt(raw.name, opt, i, kind)),
                },
                RawProfileLine::Data(data, i) => {
                    let path = Self::data_ctx(raw.name, data, i, kind)?;
                    let entry = RunnerEntry {
                        path,
                        policy,
                        stdin,
                    };
                    entries.push(entry);
                }
            }
        }
        let id = raw.id.map(RelPathStr::from_str).transpose()?;
        let kind = ProfileKind::Runner(Runner { entries });
        Ok(Profile { id, kind })
    }

    // packaged error messages
    fn data_ctx(name: &str, data: &str, i: usize, kind: &str) -> anyhow::Result<RelPathStr> {
        RelPathStr::from_str(data).with_context(|| {
            format!("Invalid data '{data}' for {kind} profile '{name}' (line {i})")
        })
    }
    fn err_opt(name: &str, opt: &str, i: usize, kind: &str) -> anyhow::Error {
        let mut opt_split = opt.split_whitespace();
        let opt1 = opt_split.next().unwrap_or("");
        anyhow!("Option '{opt1}' for {kind} profile '{name}' (line {i}) is invalid")
    }
    fn err_val(name: &str, opt: &str, i: usize, kind: &str) -> anyhow::Error {
        let mut opt_split = opt.split_whitespace();
        let opt1 = opt_split.next().unwrap_or("");
        let opt2 = opt_split.next().unwrap_or("");
        anyhow!("Option'{opt1}' for {kind} profile '{name}' (line {i}) has invalid value '{opt2}'")
    }
    fn err_dup(name: &str, opt: &str, i: usize) -> anyhow::Error {
        anyhow!("Option '{opt}' for profile '{name}' (line {i}) is duplicated")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prof::ProfileKind;

    #[test]
    fn parse_composite() -> anyhow::Result<()> {
        let expected = Profile {
            id: None,
            kind: ProfileKind::Composite(Composite {
                entries: vec![
                    CompositeEntry {
                        child: "child1".parse()?,
                    },
                    CompositeEntry {
                        child: "child2".parse()?,
                    },
                ],
            }),
        };
        let actual_config = r#"
            /! kind composite
            child1
            // This is a comment
            child2
        "#;
        let actual = Profile::parse_config(actual_config, "my_composite")?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn parse_module() -> anyhow::Result<()> {
        let expected = Profile {
            id: Some("test_module".parse()?),
            kind: ProfileKind::Module(Module {
                entries: vec![
                    ModuleEntry {
                        path: "src_main.rs".parse()?,
                        policy: ModulePolicy::Include,
                    },
                    ModuleEntry {
                        path: "src_lib.rs".parse()?,
                        policy: ModulePolicy::Include,
                    },
                    ModuleEntry {
                        path: "target".parse()?,
                        policy: ModulePolicy::Exclude,
                    },
                    ModuleEntry {
                        path: "Cargo.lock".parse()?,
                        policy: ModulePolicy::NotDiff,
                    },
                ],
                cleanup: vec![
                    RelPathStr::from_str(".cargo")?,
                    RelPathStr::from_str(".rustup")?,
                ],
            }),
        };
        let actual_config = r#"
            /! kind module
            /! id test_module
            /! policy include
            src_main.rs
            src_lib.rs
            /! policy exclude
            target
            /! policy notdiff
            Cargo.lock
            /! cleanup .cargo
            /! cleanup .rustup
        "#;
        let actual = Profile::parse_config(actual_config, "my_module")?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn parse_runner() -> anyhow::Result<()> {
        let expected = Profile {
            id: Some("test_runner".parse()?),
            kind: ProfileKind::Runner(Runner {
                entries: vec![
                    RunnerEntry {
                        path: "script1.sh".parse()?,
                        policy: RunnerPolicy::Include,
                        stdin: false,
                    },
                    RunnerEntry {
                        path: "scripts".parse()?,
                        policy: RunnerPolicy::Include,
                        stdin: false,
                    },
                    RunnerEntry {
                        path: "data".parse()?,
                        policy: RunnerPolicy::Exclude,
                        stdin: true,
                    },
                    RunnerEntry {
                        path: "script2.sh".parse()?,
                        policy: RunnerPolicy::Include,
                        stdin: true,
                    },
                ],
            }),
        };
        let actual_config = r#"
            /! kind runner
            /! id test_runner
            /! policy include
            script1.sh
            scripts
            /! stdin on
            /! policy exclude
            data
            /! policy include
            script2.sh
        "#;
        let actual = Profile::parse_config(actual_config, "my_runner")?;

        assert_eq!(actual, expected);

        Ok(())
    }
}

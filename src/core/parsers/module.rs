use crate::core::{
    error::{ErrorType, Result},
    fs::{self, PathType, RelPath},
    parsers::{RawItem, RawKind},
    profile::{
        Profile, ProfileType,
        module::{Module, ModuleEntry, ModulePolicy},
    },
};

#[derive(Debug)]
pub(super) struct ModuleParser {}

impl ModuleParser {
    pub(super) fn parse(
        profile: String,
        raw: impl Iterator<Item = Result<RawItem>>,
    ) -> Result<Profile> {
        let mut entries = vec![];
        let mut policy = ModulePolicy::default();
        let mut backup_dir = None;

        for line in raw {
            let line = line?;
            match line.kind {
                // option lines, aka module policy
                RawKind::Option => match line.content.as_str() {
                    "policy track" => policy = ModulePolicy::Track,
                    "policy notdiff" => policy = ModulePolicy::NotDiff,
                    "policy ignore" => policy = ModulePolicy::Ignore,
                    dir if dir.starts_with("dir") => {
                        let path = dir
                            .strip_prefix("dir")
                            .expect("string must start with dir")
                            .trim();
                        if fs::check_has_parent_dirs(path) {
                            return Err(ErrorType::InvalidOptionLine(
                                profile,
                                line.line,
                                line.content,
                                "dir path cannot contain parent directories".into(),
                            )
                            .into());
                        }
                        if PathType::from(path) == PathType::Absolute {
                            return Err(ErrorType::InvalidOptionLine(
                                profile,
                                line.line,
                                line.content,
                                "dir path cannot be absolute".into(),
                            )
                            .into());
                        }
                        backup_dir = Some(RelPath::from(path));
                    }
                    _ => {
                        return Err(ErrorType::InvalidOptionLine(
                            profile,
                            line.line,
                            line.content,
                            "".into(),
                        )
                        .into());
                    }
                },

                // data lines, aka relative file paths here
                RawKind::Data => {
                    if fs::check_has_parent_dirs(&line.content) {
                        return Err(ErrorType::InvalidDataLine(
                            profile,
                            line.line,
                            line.content,
                            "module paths cannot contain parent directories".into(),
                        )
                        .into());
                    }
                    let path = RelPath::from(line.content.as_str());
                    let entry = ModuleEntry::new(path, policy);
                    entries.push(entry);
                }
            }
        }

        if let Some(backup_dir) = backup_dir {
            Ok(Profile::new(
                profile,
                ProfileType::Module(Module::new(entries, backup_dir)),
            ))
        } else {
            Err(ErrorType::MissingOptionLine("dir".into()).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::parsers::{RawItem, RawKind};
    use crate::core::profile::module::ModulePolicy;

    #[test]
    fn test_parse_module() -> Result<()> {
        let raw = vec![
            RawItem {
                line: 1,
                content: "dir testing69".into(),
                kind: RawKind::Option,
            },
            RawItem {
                line: 2,
                content: "policy track".into(),
                kind: RawKind::Option,
            },
            RawItem {
                line: 3,
                content: "src/lib.rs".into(),
                kind: RawKind::Data,
            },
            RawItem {
                line: 4,
                content: "policy ignore".into(),
                kind: RawKind::Option,
            },
            RawItem {
                line: 5,
                content: "target/".into(),
                kind: RawKind::Data,
            },
        ];

        let profile = ModuleParser::parse("test".into(), raw.into_iter().map(Ok))?;

        match profile.ptype() {
            ProfileType::Module(module) => {
                let entries = module.entries();
                assert_eq!(entries.len(), 2);
                assert_eq!(entries[0].path().to_str_lossy(), "src/lib.rs");
                assert_eq!(entries[0].policy(), ModulePolicy::Track);
                assert_eq!(entries[1].path().to_str_lossy(), "target/");
                assert_eq!(entries[1].policy(), ModulePolicy::Ignore);
                assert_eq!(module.backup_dir(), &RelPath::from("testing69"))
            }
            _ => panic!("Expected Module profile type"),
        }

        Ok(())
    }
}

use crate::core::{
    error::{ErrorType, Result},
    fs::{self, RelPath},
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

        for line in raw {
            let line = line?;
            match line.kind {
                // option lines, aka module policy
                RawKind::Option => match line.content.as_str() {
                    "policy track" => policy = ModulePolicy::Track,
                    "policy notdiff" => policy = ModulePolicy::NotDiff,
                    "policy ignore" => policy = ModulePolicy::Ignore,
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

        Ok(Profile::new(
            profile,
            ProfileType::Module(Module::new(entries)),
        ))
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
                content: "policy track".into(),
                kind: RawKind::Option,
            },
            RawItem {
                line: 2,
                content: "src/lib.rs".into(),
                kind: RawKind::Data,
            },
            RawItem {
                line: 3,
                content: "policy ignore".into(),
                kind: RawKind::Option,
            },
            RawItem {
                line: 4,
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
            }
            _ => panic!("Expected Module profile type"),
        }

        Ok(())
    }
}

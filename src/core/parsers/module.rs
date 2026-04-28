use crate::core::{
    error::{Error, Result},
    fs::RelPath,
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
                    _ => return Err(Error::InvalidOptionLine(profile, line.line, line.content)),
                },

                // data lines, aka relative file paths here
                RawKind::Data => {
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

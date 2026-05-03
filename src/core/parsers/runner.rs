use crate::core::{
    error::{Error, Result},
    fs::RelPath,
    parsers::{RawItem, RawKind},
    profile::{
        Profile, ProfileType,
        runner::{Runner, RunnerEntry, RunnerPolicy},
    },
};

pub(super) struct RunnerParser {}

impl RunnerParser {
    pub(super) fn parse(
        profile: String,
        raw: impl Iterator<Item = Result<RawItem>>,
    ) -> Result<Profile> {
        let mut entries = vec![];
        let mut policy = RunnerPolicy::default();

        for line in raw {
            let line = line?;
            match line.kind {
                // option lines, aka module policy
                RawKind::Option => match line.content.as_str() {
                    "policy run" => policy = RunnerPolicy::Run,
                    "policy skip" => policy = RunnerPolicy::Skip,
                    _ => return Err(Error::InvalidOptionLine(profile, line.line, line.content)),
                },

                // data lines, aka relative file paths here
                RawKind::Data => {
                    let path = RelPath::from(line.content.as_str());
                    let entry = RunnerEntry::new(path, policy);
                    entries.push(entry);
                }
            }
        }

        Ok(Profile::new(
            profile,
            ProfileType::Runner(Runner::new(entries)),
        ))
    }
}

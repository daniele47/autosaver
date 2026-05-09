use crate::core::{
    error::{ErrorType, Result},
    fs::{self, RelPath},
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
                            "runner paths cannot contain parent directories".into(),
                        )
                        .into());
                    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::parsers::{RawItem, RawKind};
    use crate::core::profile::runner::RunnerPolicy;

    #[test]
    fn test_parse_runner() -> Result<()> {
        let raw = vec![
            RawItem {
                line: 1,
                content: "policy run".into(),
                kind: RawKind::Option,
            },
            RawItem {
                line: 2,
                content: "01_init.sh".into(),
                kind: RawKind::Data,
            },
            RawItem {
                line: 3,
                content: "02_flatpak.sh".into(),
                kind: RawKind::Data,
            },
            RawItem {
                line: 4,
                content: "policy skip".into(),
                kind: RawKind::Option,
            },
            RawItem {
                line: 5,
                content: "other/".into(),
                kind: RawKind::Data,
            },
        ];

        let profile = RunnerParser::parse("test".into(), raw.into_iter().map(Ok))?;

        match profile.ptype() {
            ProfileType::Runner(runner) => {
                let entries = runner.entries();
                assert_eq!(entries.len(), 3);

                assert_eq!(entries[0].path().to_str_lossy(), "01_init.sh");
                assert_eq!(*entries[0].policy(), RunnerPolicy::Run);

                assert_eq!(entries[1].path().to_str_lossy(), "02_flatpak.sh");
                assert_eq!(*entries[1].policy(), RunnerPolicy::Run);

                assert_eq!(entries[2].path().to_str_lossy(), "other/");
                assert_eq!(*entries[2].policy(), RunnerPolicy::Skip);
            }
            _ => panic!("Expected Runner profile type"),
        }

        Ok(())
    }
}

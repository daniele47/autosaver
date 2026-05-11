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
        let mut run_dir = None;

        for line in raw {
            let line = line?;
            match line.kind {
                // option lines, aka module policy
                RawKind::Option => match line.content.as_str() {
                    "policy run" => policy = RunnerPolicy::Run,
                    "policy skip" => policy = RunnerPolicy::Skip,
                    dir if dir.starts_with("dir") => {
                        let path = dir
                            .strip_prefix("dir")
                            .expect("string must start with dir")
                            .trim();
                        run_dir = Some(RelPath::from(path));
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

        if let Some(run_dir) = run_dir {
            Ok(Profile::new(
                profile,
                ProfileType::Runner(Runner::new(entries, run_dir)),
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
                content: "dir __whatever__".into(),
                kind: RawKind::Option,
            },
            RawItem {
                line: 3,
                content: "01_init.sh".into(),
                kind: RawKind::Data,
            },
            RawItem {
                line: 4,
                content: "02_flatpak.sh".into(),
                kind: RawKind::Data,
            },
            RawItem {
                line: 5,
                content: "policy skip".into(),
                kind: RawKind::Option,
            },
            RawItem {
                line: 6,
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
                assert_eq!(runner.run_dir(), &RelPath::from("__whatever__"))
            }
            _ => panic!("Expected Runner profile type"),
        }

        Ok(())
    }
}

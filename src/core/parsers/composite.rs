use crate::core::{
    error::{Error, Result},
    parsers::{RawItem, RawKind},
    profile::{Profile, ProfileType, composite::Composite},
};

#[derive(Debug)]
pub(super) struct CompositeParser {}

impl CompositeParser {
    pub(super) fn parse(
        profile: String,
        raw: impl Iterator<Item = Result<RawItem>>,
    ) -> Result<Profile> {
        let mut entries = vec![];

        for line in raw {
            let line = line?;
            match line.kind {
                // composite profile has NO options lines
                RawKind::Option => {
                    return Err(Error::InvalidOptionLine(profile, line.line, line.content));
                }

                // normal data lines, aka profile names here
                RawKind::Data => {
                    entries.push(line.content);
                }
            }
        }

        Ok(Profile::new(
            profile,
            ProfileType::Composite(Composite::new(entries)),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::parsers::{RawItem, RawKind};

    #[test]
    fn test_parse_composite() -> Result<()> {
        let raw = vec![
            RawItem {
                line: 1,
                content: "profile1".into(),
                kind: RawKind::Data,
            },
            RawItem {
                line: 2,
                content: "profile2".into(),
                kind: RawKind::Data,
            },
        ];

        let profile = CompositeParser::parse("test".into(), raw.into_iter().map(Ok))?;

        match profile.ptype() {
            ProfileType::Composite(composite) => {
                let entries = composite.entries();
                assert_eq!(entries.len(), 2);
                assert_eq!(entries[0], "profile1");
                assert_eq!(entries[1], "profile2");
            }
            _ => panic!("Expected Module profile type"),
        }

        Ok(())
    }
}

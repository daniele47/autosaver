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
                    if !line
                        .content
                        .chars()
                        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
                    {
                        // limit valid profile names!
                        return Err(Error::InvalidDataLine(profile, line.line, line.content));
                    }
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

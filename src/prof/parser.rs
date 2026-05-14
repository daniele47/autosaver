use tracing::instrument;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RawProfileLine<'a> {
    Option(&'a str),
    Data(&'a str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawProfile<'a> {
    lines: Vec<RawProfileLine<'a>>,
}

impl<'a> RawProfile<'a> {
    #[instrument(ret, level = "trace")]
    pub fn parse_config(config: &'a str) -> Self {
        let mut lines = Vec::new();

        for line in config.lines() {
            if let Some(option) = line.strip_prefix("/!") {

                lines.push(RawProfileLine::Option(option.trim()));
            } else if !line.starts_with("/") {
                lines.push(RawProfileLine::Data(line.trim()));
            }
        }

        Self { lines }
    }
}

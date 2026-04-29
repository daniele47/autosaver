//! Module to parse cmdline flags and splits flags from not flag parameters.

/// All possible types of a flag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Flag {
    /// Single letter such as `-a`.
    Letter(char),
    /// Word flags such as `--banana`.
    Word(String),
}

/// Represent an entire parsed cmdline.
#[derive(Debug)]
pub struct ParsedArgs {
    flags: Vec<Flag>,
    params: Vec<String>,
}

impl ParsedArgs {
    /// Get flags.
    pub fn flags(&self) -> &[Flag] {
        &self.flags
    }

    /// Get parameters.
    pub fn params(&self) -> &[String] {
        &self.params
    }

    /// Parse cmdline into Flags and parameters.
    pub fn parse(args: &[&str]) -> Self {
        let mut parsed = ParsedArgs {
            flags: Default::default(),
            params: vec![],
        };

        let mut index = 0;
        for arg in args {
            index += 1;
            if *arg == "--" {
                break;
            }
            if let Some(wflag) = arg.strip_prefix("--") {
                parsed.flags.push(Flag::Word(wflag.to_string()));
            } else if let Some(lflag) = arg.strip_prefix("-") {
                let chars = lflag.chars().map(Flag::Letter).collect::<Vec<_>>();
                parsed.flags.extend(chars);
            } else {
                parsed.params.push(arg.to_string());
            }
        }

        while let Some(rem) = args.get(index) {
            parsed.params.push(rem.to_string());
            index += 1;
        }

        parsed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let args = vec![
            "program",
            "--verbose",
            "-abc",
            "file.txt",
            "---weird",
            "--output",
            "-x",
            "data.json",
            "--",
            "--keep",
        ];
        let parsed = ParsedArgs::parse(&args[1..]);

        // Test specific flags
        assert_eq!(parsed.flags[0], Flag::Word("verbose".to_string()));
        assert_eq!(parsed.flags[1], Flag::Letter('a'));
        assert_eq!(parsed.flags[2], Flag::Letter('b'));
        assert_eq!(parsed.flags[3], Flag::Letter('c'));
        assert_eq!(parsed.flags[4], Flag::Word("-weird".to_string())); // ---weird becomes -weird
        assert_eq!(parsed.flags[5], Flag::Word("output".to_string()));
        assert_eq!(parsed.flags[6], Flag::Letter('x'));

        // Test params count and values
        assert_eq!(parsed.params.len(), 3);
        assert_eq!(parsed.params[0], "file.txt");
        assert_eq!(parsed.params[1], "data.json");
        assert_eq!(parsed.params[2], "--keep");
    }
}

use crate::{cli::ctx::CliContext, inputln, outln, outnow};

use bitflags::bitflags;
use owo_colors::OwoColorize;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PromptFlags: u32 {
        /// Answer yes
        const YES = 1 << 0;
        /// Answer no
        const NO = 1 << 1;
        /// Quit program entirely
        const QUIT = 1 << 2;
        /// Show help for the prompt
        const HELP = 1 << 3;
        /// Show the diff of 2 files
        const DIFF = 1 << 4;
        /// Open a tui editor on the file
        const EDIT = 1 << 5;
        /// Show entire file
        const SHOW = 1 << 6;

        /// Combination of flags that are always ok to have
        const BASIC = 0b1111;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prompt {
    flags: PromptFlags,
    fmt: String,
    buf: String,
}

impl Prompt {
    pub fn new(flags: PromptFlags) -> Self {
        Self {
            flags,
            fmt: Self::ordered_flags(&flags),
            buf: String::new(),
        }
    }

    fn parse_flag(input: &str) -> Option<PromptFlags> {
        match input {
            "y" => Some(PromptFlags::YES),
            "n" => Some(PromptFlags::NO),
            "d" => Some(PromptFlags::DIFF),
            "e" => Some(PromptFlags::EDIT),
            "s" => Some(PromptFlags::SHOW),
            "q" => Some(PromptFlags::QUIT),
            "h" => Some(PromptFlags::HELP),
            _ => None,
        }
    }

    fn ordered_flags(flags: &PromptFlags) -> String {
        const FLAG_LIST: &[(PromptFlags, &str)] = &[
            (PromptFlags::DIFF, "d"),
            (PromptFlags::EDIT, "e"),
            (PromptFlags::SHOW, "s"),
            (PromptFlags::QUIT, "q"),
            (PromptFlags::YES, "y"),
            (PromptFlags::NO, "n"),
            (PromptFlags::HELP, "h"),
        ];
        let mut res = [""; 7];
        let mut count = 0;
        for (flag, ch) in FLAG_LIST {
            if flags.contains(*flag) {
                res[count] = ch;
                count += 1;
            }
        }
        res[..count].join("/")
    }

    pub fn prompt(&mut self, msg: &str) -> PromptFlags {
        loop {
            outnow!("{} [{}] ", msg.style(CliContext::PROMPT_MSG), self.fmt);
            let input = inputln!(&mut self.buf);
            if let Some(input) = Self::parse_flag(input) {
                return input;
            }
            outln!("Invalid flag passed. Please retry...")
        }
    }
}

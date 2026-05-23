use crate::{cli::ctx::CliContext, fs::abs::AbsPathStr, inputln, outln, outnow, warning};

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
            (PromptFlags::HELP, "h"),
            (PromptFlags::NO, "n"),
            (PromptFlags::QUIT, "q"),
            (PromptFlags::SHOW, "s"),
            (PromptFlags::YES, "y"),
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

    pub fn handled_prompt(&mut self, msg: &str, paths: &[&AbsPathStr]) -> Option<PromptFlags> {
        let prompt_flag = self.prompt(msg);
        match prompt_flag {
            i if i.contains(PromptFlags::NO) => self.on_no(),
            i if i.contains(PromptFlags::QUIT) => self.on_quit(),
            i if i.contains(PromptFlags::HELP) => self.on_help(),
            i if i.contains(PromptFlags::DIFF) => self.on_diff(paths[0], paths[1]),
            i if i.contains(PromptFlags::EDIT) => self.on_edit(paths[0]),
            i if i.contains(PromptFlags::SHOW) => self.on_show(paths[0]),
            _ => return Some(prompt_flag),
        }
        None
    }

    pub fn on_no(&self) {}
    pub fn on_quit(&self) -> ! {
        std::process::exit(0)
    }
    pub fn on_help(&self) {
        let f = self.flags;
        if f.contains(PromptFlags::DIFF) {
            outln!("[D]iff : show the diff between the files");
        }
        if f.contains(PromptFlags::EDIT) {
            outln!("[E]dit : edit the file with the $EDITOR");
        }
        if f.contains(PromptFlags::HELP) {
            outln!("[H]elp : show the current help message");
        }
        if f.contains(PromptFlags::NO) {
            outln!("[N]o   : answer no to the prompt");
        }
        if f.contains(PromptFlags::QUIT) {
            outln!("[Q]uit : quit the program entirely");
        }
        if f.contains(PromptFlags::SHOW) {
            outln!("[S]how : show the file in question");
        }
        if f.contains(PromptFlags::YES) {
            outln!("[Y]es  : answer yes to the prompt");
        }
    }
    pub fn on_edit(&self, file: &AbsPathStr) {
        let editor = std::env::var("EDITOR").ok();

        match editor {
            Some(editor_cmd) => {
                let status = std::process::Command::new(&editor_cmd)
                    .arg(file.path())
                    .status();

                match status {
                    Ok(exit_status) if exit_status.success() => {}
                    Ok(exit_status) => {
                        let code = exit_status.code().unwrap_or(-1);
                        warning!("Failed to edit '{editor_cmd}' exited with error code: {code}");
                    }
                    Err(e) => warning!("Failed to launch '{}': {}", editor_cmd, e),
                }
            }
            None => warning!("No editor found! Set EDITOR environment variable"),
        }
    }
    pub fn on_show(&self, file: &AbsPathStr) {
        match file.read_file() {
            Ok(text) => outnow!("{text}"),
            Err(e) => warning!("{e}"),
        }
    }
    pub fn on_diff(&self, ______old: &AbsPathStr, ______new: &AbsPathStr) {
        unimplemented!("on_diff")
    }
}

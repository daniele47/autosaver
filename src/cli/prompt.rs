use crate::{cli::ctx::CliContext, fs::abs::AbsPathStr, inputln, out, outln, outnow, warning};

use bitflags::bitflags;
use owo_colors::OwoColorize;
use similar::{ChangeTag, TextDiff};

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
            "n" | "" => Some(PromptFlags::NO),
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

    pub fn handled_prompt<T>(
        &mut self,
        msg: &str,
        paths: &[&AbsPathStr],
        action: T,
    ) -> anyhow::Result<()>
    where
        T: FnOnce() -> anyhow::Result<()>,
    {
        let mut prompt_flag = self.prompt(msg);
        while !prompt_flag.contains(PromptFlags::YES) && !prompt_flag.contains(PromptFlags::NO) {
            match prompt_flag {
                i if i.contains(PromptFlags::QUIT) => self.on_quit(),
                i if i.contains(PromptFlags::HELP) => self.on_help(),
                i if i.contains(PromptFlags::DIFF) => self.on_diff(paths),
                i if i.contains(PromptFlags::EDIT) => self.on_edit(paths),
                i if i.contains(PromptFlags::SHOW) => self.on_show(paths),
                _ => unimplemented!("Prompt flag not handled"),
            };
            prompt_flag = self.prompt(msg);
        }
        match prompt_flag {
            i if i.contains(PromptFlags::YES) => self.on_yes(action)?,
            i if i.contains(PromptFlags::NO) => self.on_no(),
            _ => unimplemented!("Prompt flag not handled"),
        }
        Ok(())
    }

    pub fn on_no(&self) {}
    pub fn on_yes<T>(&self, action: T) -> anyhow::Result<()>
    where
        T: FnOnce() -> anyhow::Result<()>,
    {
        action()
    }
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
    pub fn on_edit(&self, paths: &[&AbsPathStr]) {
        match std::env::var("EDITOR").ok() {
            Some(editor_cmd) => {
                let cmd = std::process::Command::new(&editor_cmd)
                    .args(paths.iter().map(|p| p.path()))
                    .status();

                match cmd {
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
    pub fn on_show(&self, paths: &[&AbsPathStr]) {
        for path in paths {
            let header = format!("@@ {} @@", path.display());
            outln!("{}", header.style(CliContext::SHOW_HEADER));
            match path.read_file() {
                Ok(text) => outnow!("{text}"),
                Err(e) => warning!("{e}"),
            }
        }
    }
    pub fn on_diff(&self, paths: &[&AbsPathStr]) {
        assert_eq!(paths.len(), 2);
        let old_text = paths[0].read_file();
        let new_text = paths[1].read_file();

        if let (Ok(old_text), Ok(new_text)) = (&old_text, &new_text) {
            let diff = TextDiff::from_lines(old_text, new_text);
            let groups = diff.grouped_ops(3);

            for group in groups {
                // Calculate line ranges for this hunk
                if let Some(first_op) = group.first()
                    && let Some(last_op) = group.last()
                {
                    let old_start = first_op.old_range().start + 1;
                    let old_end = last_op.old_range().end;
                    let old_len = old_end - old_start + 1;
                    let new_start = first_op.new_range().start + 1;
                    let new_end = last_op.new_range().end;
                    let new_len = new_end - new_start + 1;

                    // Print the hunk header
                    let str = format!(
                        "@@ -{},{} +{},{} @@",
                        old_start, old_len, new_start, new_len
                    );
                    outln!("{}", str.style(CliContext::DIFF_HEADER));
                }

                for op in group {
                    for change in diff.iter_changes(&op) {
                        match change.tag() {
                            ChangeTag::Delete => {
                                out!("{} {change}", "-".style(CliContext::DIFF_DELETED))
                            }
                            ChangeTag::Insert => {
                                out!("{} {change}", "+".style(CliContext::DIFF_INSERTED))
                            }
                            ChangeTag::Equal => out!("  {change}"),
                        };
                    }
                }
            }
            outnow!(); // force a safety flush
        } else {
            if let Err(e) = old_text {
                warning!("{}", e)
            }
            if let Err(e) = new_text {
                warning!("{}", e)
            }
        }
    }
}

use crate::{
    cli::{ctx::CliContext, error::EarlyQuit},
    fs::abs::AbsPathStr,
    inputln, out, outln, outnow, warning,
};

use anyhow::bail;
use bitflags::bitflags;
use owo_colors::OwoColorize;
use similar::{ChangeTag, TextDiff};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PromptAnswer: u32 {
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
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PromptFlags {
    answer_no: bool,
    answer_yes: bool,
    skip_prompt: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prompt {
    allowed_answers: PromptAnswer,
    flags: PromptFlags,
    fmt: String,
    buf: String,
}

impl PromptFlags {
    pub fn new(answer_no: bool, answer_yes: bool, skip_prompt: bool) -> Self {
        Self {
            answer_no,
            answer_yes,
            skip_prompt,
        }
    }
}

impl Prompt {
    pub fn new(allowed_answers: PromptAnswer, flags: PromptFlags) -> Self {
        let allowed_answers = allowed_answers | PromptAnswer::YES | PromptAnswer::NO;
        Self {
            allowed_answers,
            flags,
            fmt: Self::ordered_answers(&allowed_answers),
            buf: String::new(),
        }
    }

    fn parse_answer(input: &str, allowed: PromptAnswer) -> Option<PromptAnswer> {
        match input {
            "y" => Some(PromptAnswer::YES),
            "n" | "" => Some(PromptAnswer::NO),
            "d" => Some(PromptAnswer::DIFF),
            "e" => Some(PromptAnswer::EDIT),
            "s" => Some(PromptAnswer::SHOW),
            "q" => Some(PromptAnswer::QUIT),
            "h" => Some(PromptAnswer::HELP),
            _ => None,
        }
        .map(|f| f & allowed)
        .and_then(|f| if f.is_empty() { None } else { Some(f) })
    }

    fn ordered_answers(answers: &PromptAnswer) -> String {
        const ANSWER_LIST: &[(PromptAnswer, &str)] = &[
            (PromptAnswer::DIFF, "d"),
            (PromptAnswer::EDIT, "e"),
            (PromptAnswer::HELP, "h"),
            (PromptAnswer::NO, "n"),
            (PromptAnswer::QUIT, "q"),
            (PromptAnswer::SHOW, "s"),
            (PromptAnswer::YES, "y"),
        ];
        let mut res = [""; 7];
        let mut count = 0;
        for (answer, ch) in ANSWER_LIST {
            if answers.contains(*answer) {
                res[count] = ch;
                count += 1;
            }
        }
        res[..count].join("/")
    }

    pub fn prompt(&mut self, msg: &str) -> PromptAnswer {
        loop {
            if self.flags.skip_prompt {
                return PromptAnswer::NO;
            }
            let msg = msg.style(CliContext::PROMPT_MSG);
            let choises = format!("[{}]", self.fmt);
            let choises = choises.style(CliContext::PROMPT_CHOICES);
            outnow!("{msg} {choises} ",);
            if self.flags.answer_no {
                outln!("n");
                return PromptAnswer::NO;
            }
            if self.flags.answer_yes {
                outln!("y");
                return PromptAnswer::YES;
            }
            let input = inputln!(&mut self.buf);
            if let Some(input) = Self::parse_answer(input, self.allowed_answers) {
                return input;
            }
            outln!("Invalid answer '{input}'. Please retry...")
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
        let mut prompt_answer = self.prompt(msg);
        while !prompt_answer.contains(PromptAnswer::YES)
            && !prompt_answer.contains(PromptAnswer::NO)
        {
            match prompt_answer {
                i if i.contains(PromptAnswer::QUIT) => self.on_quit()?,
                i if i.contains(PromptAnswer::HELP) => self.on_help(),
                i if i.contains(PromptAnswer::DIFF) => self.on_diff(paths),
                i if i.contains(PromptAnswer::EDIT) => self.on_edit(paths),
                i if i.contains(PromptAnswer::SHOW) => self.on_show(paths),
                _ => unimplemented!("Prompt answer not handled"),
            };
            prompt_answer = self.prompt(msg);
        }
        match prompt_answer {
            i if i.contains(PromptAnswer::YES) => self.on_yes(action)?,
            i if i.contains(PromptAnswer::NO) => self.on_no(),
            _ => unimplemented!("Prompt answer not handled"),
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
    pub fn on_quit(&self) -> anyhow::Result<()> {
        bail!(EarlyQuit)
    }
    pub fn on_help(&self) {
        let f = self.allowed_answers;
        if f.contains(PromptAnswer::DIFF) {
            outln!("[D]iff : show the diff between the files");
        }
        if f.contains(PromptAnswer::EDIT) {
            outln!("[E]dit : edit the file with the $EDITOR");
        }
        if f.contains(PromptAnswer::HELP) {
            outln!("[H]elp : show the current help message");
        }
        if f.contains(PromptAnswer::NO) {
            outln!("[N]o   : answer no to the prompt");
        }
        if f.contains(PromptAnswer::QUIT) {
            outln!("[Q]uit : quit the program entirely");
        }
        if f.contains(PromptAnswer::SHOW) {
            outln!("[S]how : show the file in question");
        }
        if f.contains(PromptAnswer::YES) {
            outln!("[Y]es  : answer yes to the prompt");
        }
    }
    pub fn on_edit(&self, paths: &[&AbsPathStr]) {
        assert!(!paths.is_empty());
        match std::env::var("EDITOR").ok() {
            Some(editor_cmd) => {
                let cmd = std::process::Command::new(&editor_cmd)
                    .args(paths.iter().map(|p| p.path()))
                    .status();

                match cmd {
                    Ok(exit_status) if exit_status.success() => {}
                    Ok(exit_status) => {
                        let code = exit_status.code().unwrap_or(-1);
                        warning!("Editor '{editor_cmd}' exited with error code: {code}");
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

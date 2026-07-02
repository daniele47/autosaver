use crate::{
    cli::{EarlyQuit, config::col::CliColor},
    fs::abs::AbsPathStr,
    out, outln, outnow, warning,
};

use anyhow::{Context, bail};
use owo_colors::OwoColorize;
use similar::{ChangeTag, TextDiff};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PromptAnswer {
    Yes = 1 << 0,  // execute what prompt asks for
    No = 1 << 1,   // not execute what prompt asks for
    Quit = 1 << 2, // quit program entirely
    Help = 1 << 3, // show help about answers
    Diff = 1 << 4, // show diff between two files
    Edit = 1 << 5, // edit all files
    Show = 1 << 6, // show files in their entirety
    Full = 1 << 7, // show full path of all files
}
type PromptAnswers = u8;

#[derive(Debug, Clone, PartialEq)]
pub struct Prompt {
    auto_answers: Vec<PromptAnswer>,
    auto_skip: bool,
}

impl Prompt {
    pub fn new(
        auto_answers: String,
        auto_yes: bool,
        auto_no: bool,
        auto_skip: bool,
    ) -> anyhow::Result<Self> {
        let auto_answers = Self::auto_answers(auto_answers, auto_yes, auto_no)?;
        Ok(Self {
            auto_answers,
            auto_skip,
        })
    }

    fn parse_answer(input: char) -> Option<PromptAnswer> {
        match input {
            'd' => Some(PromptAnswer::Diff),
            'e' => Some(PromptAnswer::Edit),
            'f' => Some(PromptAnswer::Full),
            'h' => Some(PromptAnswer::Help),
            'n' => Some(PromptAnswer::No),
            'q' => Some(PromptAnswer::Quit),
            's' => Some(PromptAnswer::Show),
            'y' => Some(PromptAnswer::Yes),
            _ => None,
        }
    }

    fn auto_answers(
        input: String,
        auto_yes: bool,
        auto_no: bool,
    ) -> anyhow::Result<Vec<PromptAnswer>> {
        let mut answers = vec![];
        let allowed_auto_answers = &[PromptAnswer::Diff, PromptAnswer::Full, PromptAnswer::Show];
        for c in input.chars() {
            let parsed_char = Self::parse_answer(c)
                .with_context(|| format!("Unknown answer '{c}' inside auto-answer: '{input}'"))?;
            if !allowed_auto_answers.contains(&parsed_char) {
                bail!(format!("Answer '{c}' is not allowed as an auto-answer"));
            }
            answers.push(parsed_char);
        }
        if auto_no {
            answers.push(PromptAnswer::No);
        }
        if auto_yes {
            answers.push(PromptAnswer::Yes);
        }
        Ok(answers)
    }

    fn ordered_answers(answers: PromptAnswers) -> String {
        const ANSWER_LIST: &[(PromptAnswer, &str)] = &[
            (PromptAnswer::Diff, "d"),
            (PromptAnswer::Edit, "e"),
            (PromptAnswer::Full, "f"),
            (PromptAnswer::Help, "h"),
            (PromptAnswer::No, "n"),
            (PromptAnswer::Quit, "q"),
            (PromptAnswer::Show, "s"),
            (PromptAnswer::Yes, "y"),
        ];
        let mut res = [""; ANSWER_LIST.len()];
        let mut count = 0;
        for (answer, ch) in ANSWER_LIST {
            if *answer as PromptAnswers & answers != 0 {
                res[count] = ch;
                count += 1;
            }
        }
        res[..count].join("/")
    }

    pub fn question(
        &self,
        msg: &str,
        paths: &[&AbsPathStr],
        action: impl FnOnce() -> anyhow::Result<()>,
        col: &CliColor,
    ) -> anyhow::Result<()> {
        // early exit with auto_skip enabled
        if self.auto_skip {
            return Ok(());
        }

        // filter invalid answers out
        let valid_answers = {
            let mut answers = PromptAnswers::MAX;
            if paths.is_empty() {
                answers &= !(PromptAnswer::Edit as PromptAnswers
                    | PromptAnswer::Show as PromptAnswers
                    | PromptAnswer::Full as PromptAnswers);
            }
            if paths.len() != 2 {
                answers &= !(PromptAnswer::Diff as PromptAnswers);
            }
            answers
        };
        Ok(())
    }

    // pub fn prompt(&self, msg: &str) -> PromptAnswer {
    //     loop {
    //         if self.flags.skip_prompt {
    //             return PromptAnswer::NO;
    //         }
    //         let msg = msg.style(self.col.prompt_msg);
    //         let choises = format!("[{}]", self.fmt);
    //         let choises = choises.style(self.col.prompt_choices);
    //         outnow!("{msg} {choises} ",);
    //         if self.flags.answer_no {
    //             outln!("n");
    //             return PromptAnswer::NO;
    //         }
    //         if self.flags.answer_yes {
    //             outln!("y");
    //             return PromptAnswer::YES;
    //         }
    //         let input = inputln!();
    //         if !input.ends_with("\n") {
    //             outln!();
    //         }
    //         let input = input.trim();
    //         if let Some(input) = Self::parse_answer(input, self.allowed_answers) {
    //             return input;
    //         }
    //         outln!("Invalid answer '{input}'. Please retry...")
    //     }
    // }
    //
    // pub fn handled_prompt<T>(
    //     &self,
    //     msg: &str,
    //     paths: &[&AbsPathStr],
    //     action: T,
    // ) -> anyhow::Result<()>
    // where
    //     T: FnOnce() -> anyhow::Result<()>,
    // {
    //     let mut prompt_answer = self.prompt(msg);
    //     while !prompt_answer.contains(PromptAnswer::YES)
    //         && !prompt_answer.contains(PromptAnswer::NO)
    //     {
    //         match prompt_answer {
    //             i if i.contains(PromptAnswer::QUIT) => self.on_quit()?,
    //             i if i.contains(PromptAnswer::HELP) => self.on_help(),
    //             i if i.contains(PromptAnswer::DIFF) => self.on_diff(paths),
    //             i if i.contains(PromptAnswer::EDIT) => self.on_edit(paths),
    //             i if i.contains(PromptAnswer::SHOW) => self.on_show(paths),
    //             i if i.contains(PromptAnswer::FULL) => self.on_full(paths),
    //             _ => unimplemented!("Prompt answer not handled"),
    //         };
    //         prompt_answer = self.prompt(msg);
    //     }
    //     match prompt_answer {
    //         i if i.contains(PromptAnswer::YES) => self.on_yes(action)?,
    //         i if i.contains(PromptAnswer::NO) => self.on_no(),
    //         _ => unimplemented!("Prompt answer not handled"),
    //     }
    //     Ok(())
    // }
    //
    // pub fn handled_prompt_available<T>(
    //     &self,
    //     msg: &str,
    //     paths: &[&AbsPathStr],
    //     action: T,
    // ) -> anyhow::Result<()>
    // where
    //     T: FnOnce() -> anyhow::Result<()>,
    // {
    //     let mut answers = self.allowed_answers;
    //     if paths.is_empty() {
    //         answers &= !(PromptAnswer::EDIT | PromptAnswer::SHOW | PromptAnswer::FULL);
    //     }
    //     if paths.len() != 2 {
    //         answers &= !PromptAnswer::DIFF;
    //     }
    //     if self.allowed_answers != answers {
    //         Self::new(answers, self.flags, self.col).handled_prompt(msg, paths, action)
    //     } else {
    //         self.handled_prompt(msg, paths, action)
    //     }
    // }

    fn on_no(&self) {}
    fn on_yes<T>(&self, action: T) -> anyhow::Result<()>
    where
        T: FnOnce() -> anyhow::Result<()>,
    {
        action()
    }
    fn on_quit(&self) -> anyhow::Result<()> {
        bail!(EarlyQuit)
    }
    fn on_full(&self, paths: &[&AbsPathStr]) {
        for path in paths {
            outln!("- {}", path.display());
        }
    }
    fn on_help(&self, ok_answers: PromptAnswers) {
        if PromptAnswer::Diff as PromptAnswers & ok_answers != 0 {
            outln!("[D]iff : show the diff between the files");
        }
        if PromptAnswer::Edit as PromptAnswers & ok_answers != 0 {
            outln!("[E]dit : edit the file with the $EDITOR");
        }
        if PromptAnswer::Full as PromptAnswers & ok_answers != 0 {
            outln!("[F]ull : show all the full paths");
        }
        if PromptAnswer::Help as PromptAnswers & ok_answers != 0 {
            outln!("[H]elp : show the current help message");
        }
        if PromptAnswer::No as PromptAnswers & ok_answers != 0 {
            outln!("[N]o   : answer no to the prompt");
        }
        if PromptAnswer::Quit as PromptAnswers & ok_answers != 0 {
            outln!("[Q]uit : quit the program entirely");
        }
        if PromptAnswer::Show as PromptAnswers & ok_answers != 0 {
            outln!("[S]how : show the file in question");
        }
        if PromptAnswer::Yes as PromptAnswers & ok_answers != 0 {
            outln!("[Y]es  : answer yes to the prompt");
        }
    }
    fn on_edit(&self, paths: &[&AbsPathStr]) {
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
    fn on_show(&self, paths: &[&AbsPathStr], col: &CliColor) {
        assert!(!paths.is_empty());
        for path in paths {
            let header = format!("@@ {} @@", path.display());
            outln!("{}", header.style(col.show_header));
            match path.read_file() {
                Ok(text) => outnow!("{text}"),
                Err(e) => warning!("{e}"),
            }
        }
    }
    fn on_diff(&self, paths: &[&AbsPathStr], col: &CliColor) {
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
                    outln!("{}", str.style(col.diff_header));
                }

                for op in group {
                    for change in diff.iter_changes(&op) {
                        match change.tag() {
                            ChangeTag::Delete => {
                                out!("{} {change}", "-".style(col.diff_deleted))
                            }
                            ChangeTag::Insert => {
                                out!("{} {change}", "+".style(col.diff_inserted))
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

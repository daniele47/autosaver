use crate::{
    cli::{EarlyQuit, config::col::CliColor},
    fs::abs::AbsPathStr,
    inputln, out, outln, outnow, warning,
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
        auto_answers: &str,
        auto_yes: bool,
        auto_no: bool,
        auto_skip: bool,
    ) -> anyhow::Result<Self> {
        // parse auto_answers
        let allowed_auto_answers = PromptAnswer::Diff as PromptAnswers
            | PromptAnswer::Full as PromptAnswers
            | PromptAnswer::Show as PromptAnswers;
        let mut auto_answers = Self::parse_answers(auto_answers, allowed_auto_answers)?;
        if auto_no {
            auto_answers.push(PromptAnswer::No);
        }
        if auto_yes {
            auto_answers.push(PromptAnswer::Yes);
        }

        // instanciate new object
        Ok(Self {
            auto_answers,
            auto_skip,
        })
    }
    pub fn question(
        &self,
        msg: &str,
        paths: &[&AbsPathStr],
        action: impl FnOnce() -> anyhow::Result<()>,
        col: &CliColor,
    ) -> anyhow::Result<()> {
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
        let sep = "@".repeat(80);

        // loop through answers
        let mut auto_answer_iter = self.auto_answers.iter();
        let mut input_answers = vec![];
        let mut input_answers_index = 0;
        loop {
            // prompt
            let msg = msg.style(col.prompt_msg);
            let choises = format!("[{}]", Self::ordered_answers(valid_answers));
            let choises = choises.style(col.prompt_choices);

            // get next answer
            let answer: Option<PromptAnswer> = {
                let mut answer = None;
                if let Some(next_auto) = auto_answer_iter.next() {
                    // use remaining automatic answers from cmdline flag
                    answer = Some(*next_auto);
                    outnow!("{msg} {choises} ");
                    outln!("{}", Self::answer_to_char(*next_auto));
                } else if let Some(next_input) = input_answers.get(input_answers_index) {
                    // use remaining input answers from stdin
                    input_answers_index += 1;
                    answer = Some(*next_input);
                    outnow!("{msg} {choises} ");
                    outln!("{}", Self::answer_to_char(*next_input));
                } else {
                    // early quit if auto_skip is enabled
                    if self.auto_skip {
                        if !self.auto_answers.is_empty() {
                            outln!("{sep}");
                        }
                        return Ok(());
                    }
                    // ask for new input
                    loop {
                        outnow!("{msg} {choises} ");
                        let input = inputln!();
                        if !input.ends_with("\n") {
                            outln!();
                        }
                        let parsed_answer = Self::parse_answers(input.trim(), valid_answers);
                        if let Ok(ok_ans) = parsed_answer {
                            input_answers = ok_ans;
                            break;
                        } else if let Err(err_ans) = parsed_answer {
                            outln!("{err_ans}. Please retry...");
                        }
                    }
                    // input_answers = Self::parse_answers(input.trim(), valid_answers)?;
                    if let Some(next_input) = input_answers.first() {
                        answer = Some(*next_input);
                    }
                    input_answers_index = 1;
                }
                answer
            };

            // check answer is currently valid
            if let Some(ans) = answer
                && ans as PromptAnswers & valid_answers == 0
            {
                let answer_char = Self::answer_to_char(answer.unwrap());
                warning!("Answer '{answer_char}' is invalid for current prompt");
                continue;
            }

            // act based on action
            match answer {
                Some(PromptAnswer::Yes) => {
                    outln!("{sep}");
                    return action();
                }
                Some(PromptAnswer::No) => {
                    outln!("{sep}");
                    return Ok(());
                }
                Some(PromptAnswer::Quit) => bail!(EarlyQuit),
                Some(PromptAnswer::Help) => self.on_help(valid_answers),
                Some(PromptAnswer::Diff) => self.on_diff(paths, col),
                Some(PromptAnswer::Edit) => self.on_edit(paths),
                Some(PromptAnswer::Show) => self.on_show(paths, col),
                Some(PromptAnswer::Full) => self.on_full(paths),
                None => {
                    outln!("{sep}");
                    return Ok(());
                }
            };
        }
    }

    // UTILITY FUNCTIONS

    const fn all_answers() -> &'static [PromptAnswer] {
        &[
            (PromptAnswer::Diff),
            (PromptAnswer::Edit),
            (PromptAnswer::Full),
            (PromptAnswer::Help),
            (PromptAnswer::No),
            (PromptAnswer::Quit),
            (PromptAnswer::Show),
            (PromptAnswer::Yes),
        ]
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
    fn answer_to_char(answer: PromptAnswer) -> char {
        match answer {
            PromptAnswer::Diff => 'd',
            PromptAnswer::Edit => 'e',
            PromptAnswer::Full => 'f',
            PromptAnswer::Help => 'h',
            PromptAnswer::No => 'n',
            PromptAnswer::Quit => 'q',
            PromptAnswer::Show => 's',
            PromptAnswer::Yes => 'y',
        }
    }
    fn parse_answers(
        answers: &str,
        valid_answers: PromptAnswers,
    ) -> anyhow::Result<Vec<PromptAnswer>> {
        let mut res = vec![];
        for c in answers.chars() {
            let parsed_char = Self::parse_answer(c)
                .with_context(|| format!("Unknown answer '{c}' inside answers: '{answers}'"))?;
            if valid_answers & parsed_char as PromptAnswers == 0 {
                let allowed = Self::ordered_answers(valid_answers);
                bail!(format!("Answer '{c}' is not allowed (not in: '{allowed}')"));
            }
            res.push(parsed_char);
        }
        Ok(res)
    }
    fn ordered_answers(answers: PromptAnswers) -> String {
        const SEP: char = '/';
        Self::all_answers()
            .iter()
            .filter(|e| **e as PromptAnswers & answers != 0)
            .map(|e| Self::answer_to_char(*e))
            .fold(String::new(), |mut acc, new| {
                if !acc.is_empty() {
                    acc.push(SEP);
                }
                acc.push(new);
                acc
            })
    }

    // UTILITY ACTION FUNCTIONS

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

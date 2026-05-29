use owo_colors::{OwoColorize, Style};

use crate::{fs::rel::RelPathStr, outln};

pub struct CliColor {}

impl CliColor {
    pub const TREE_COMPOSITE: Style = Style::new();
    pub const TREE_RUNNER: Style = Style::new().green();
    pub const TREE_MODULE: Style = Style::new().bright_blue();
    pub const TREE_DEDUP: Style = Style::new().yellow();
    pub const PROMPT_MSG: Style = Style::new().italic();
    pub const PROMPT_CHOICES: Style = Style::new().bright_black();
    pub const OUTPUT_PROFILE: Style = Style::new().purple();
    pub const OUTPUT_PATH: Style = Style::new().bright_blue();
    pub const OUTPUT_MISSING: Style = Style::new().red();
    pub const OUTPUT_DIFF: Style = Style::new().bright_yellow();
    pub const OUTPUT_UNMODIFIED: Style = Style::new().green();
    pub const DIFF_DELETED: Style = Style::new().red();
    pub const DIFF_INSERTED: Style = Style::new().green();
    pub const DIFF_HEADER: Style = Style::new().cyan();
    pub const SHOW_HEADER: Style = Style::new().cyan();

    pub fn output_profile(profile: &RelPathStr, style: Style) {
        outln!(
            "{} {} {0}",
            "***".style(style),
            profile.display().style(style)
        );
    }

    pub fn output_path(path: &RelPathStr, style: Style) {
        outln!("{} {}", "-".style(style), path.display().style(style));
    }
}

use owo_colors::{OwoColorize, Style};

use crate::{fs::rel::RelPathStr, outln};

#[derive(Debug, Clone, PartialEq)]
pub struct CliColor {
    pub tree_composite: Style,
    pub tree_runner: Style,
    pub tree_module: Style,
    pub tree_dedup: Style,
    pub prompt_msg: Style,
    pub prompt_choices: Style,
    pub output_profile: Style,
    pub output_path: Style,
    pub output_missing: Style,
    pub output_diff: Style,
    pub output_unmodified: Style,
    pub diff_deleted: Style,
    pub diff_inserted: Style,
    pub diff_header: Style,
    pub show_header: Style,
}

impl CliColor {
    pub fn default_theme() -> Self {
        Self {
            tree_composite: Style::new(),
            tree_runner: Style::new().green(),
            tree_module: Style::new().bright_blue(),
            tree_dedup: Style::new().yellow(),
            prompt_msg: Style::new().italic(),
            prompt_choices: Style::new().bright_black(),
            output_profile: Style::new().purple(),
            output_path: Style::new().bright_blue(),
            output_missing: Style::new().red(),
            output_diff: Style::new().bright_yellow(),
            output_unmodified: Style::new().green(),
            diff_deleted: Style::new().red(),
            diff_inserted: Style::new().green(),
            diff_header: Style::new().cyan(),
            show_header: Style::new().cyan(),
        }
    }

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

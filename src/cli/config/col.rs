use anyhow::{Ok, bail};
use owo_colors::{OwoColorize, Style};

use crate::{
    fs::{abs::AbsPathStr, path::PathStr, rel::RelPathStr},
    outln,
};

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
            prompt_msg: Style::new().italic().bright_black().underline(),
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

    pub fn output_profile(&self, profile: &RelPathStr) {
        let style = self.output_profile;
        let profile = profile.display();
        let profile = profile.style(style);
        outln!("{} {profile} {0}", "***".style(style));
    }

    pub fn output_path(&self, path: impl AsRef<PathStr>, style: Style) {
        let path = path.as_ref();
        outln!("{} {}", "-".style(style), path.display().style(style));
    }

    pub fn parse_theme(colors_file: &AbsPathStr) -> anyhow::Result<Self> {
        let mut colors = Self::default_theme();
        // quit on missing config file
        if !colors_file.is_file() {
            return Ok(colors);
        }

        for (i, line) in colors_file.read_file()?.lines().enumerate() {
            // skip comments
            if line.starts_with("#") {
                continue;
            }
            // split by whitespace, and consider only lines with at least 1 word
            let mut words = line.split_whitespace();
            if let Some(element) = words.next() {
                let mut style = Style::new();
                while let Some(style_word) = words.next() {
                    match style_word {
                        w => {
                            let p = colors_file.display();
                            bail!(
                                "Line {i} of colors config file ({p}) contains invalid style '{w}'"
                            )
                        }
                    }
                }
                match element {
                    "tree_composite" => colors.tree_composite = style,
                    "tree_runner" => colors.tree_runner = style,
                    "tree_module" => colors.tree_module = style,
                    "tree_dedup" => colors.tree_dedup = style,
                    "prompt_msg" => colors.prompt_msg = style,
                    "prompt_choices" => colors.prompt_choices = style,
                    "output_profile" => colors.output_profile = style,
                    "output_path" => colors.output_path = style,
                    "output_missing" => colors.output_missing = style,
                    "output_diff" => colors.output_diff = style,
                    "output_unmodified" => colors.output_unmodified = style,
                    "diff_deleted" => colors.diff_deleted = style,
                    "diff_inserted" => colors.diff_inserted = style,
                    "diff_header" => colors.diff_header = style,
                    "show_header" => colors.show_header = style,
                    w => {
                        let p = colors_file.display();
                        bail!("Line {i} of colors config file ({p}) contains invalid element '{w}'")
                    }
                }
            }
        }

        Ok(colors)
    }
}

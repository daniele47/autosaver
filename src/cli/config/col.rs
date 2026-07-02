use anyhow::bail;
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
    pub output_create: Style,
    pub output_delete: Style,
    pub output_missing: Style,
    pub output_diff: Style,
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
            output_create: Style::new().green(),
            output_delete: Style::new().red(),
            output_missing: Style::new().red(),
            output_diff: Style::new().bright_yellow(),
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
            let i = i + 1;
            // skip comments
            if line.starts_with("#") {
                continue;
            }
            // split by whitespace, and consider only lines with at least 1 word
            let mut words = line.split_whitespace();
            if let Some(element) = words.next() {
                let mut style = Style::new();
                for style_word in words {
                    match style_word {
                        "black" => style = style.black(),
                        "red" => style = style.red(),
                        "green" => style = style.green(),
                        "yellow" => style = style.yellow(),
                        "blue" => style = style.blue(),
                        "magenta" => style = style.magenta(),
                        "purple" => style = style.purple(),
                        "cyan" => style = style.cyan(),
                        "white" => style = style.white(),
                        "bright_black" => style = style.bright_black(),
                        "bright_red" => style = style.bright_red(),
                        "bright_green" => style = style.bright_green(),
                        "bright_yellow" => style = style.bright_yellow(),
                        "bright_blue" => style = style.bright_blue(),
                        "bright_magenta" => style = style.bright_magenta(),
                        "bright_purple" => style = style.bright_purple(),
                        "bright_cyan" => style = style.bright_cyan(),
                        "bright_white" => style = style.bright_white(),
                        "on_black" => style = style.on_black(),
                        "on_red" => style = style.on_red(),
                        "on_green" => style = style.on_green(),
                        "on_yellow" => style = style.on_yellow(),
                        "on_blue" => style = style.on_blue(),
                        "on_magenta" => style = style.on_magenta(),
                        "on_purple" => style = style.on_purple(),
                        "on_cyan" => style = style.on_cyan(),
                        "on_white" => style = style.on_white(),
                        "on_bright_black" => style = style.on_bright_black(),
                        "on_bright_red" => style = style.on_bright_red(),
                        "on_bright_green" => style = style.on_bright_green(),
                        "on_bright_yellow" => style = style.on_bright_yellow(),
                        "on_bright_blue" => style = style.on_bright_blue(),
                        "on_bright_magenta" => style = style.on_bright_magenta(),
                        "on_bright_purple" => style = style.on_bright_purple(),
                        "on_bright_cyan" => style = style.on_bright_cyan(),
                        "on_bright_white" => style = style.on_bright_white(),
                        "bold" => style = style.bold(),
                        "dimmed" => style = style.dimmed(),
                        "italic" => style = style.italic(),
                        "underline" => style = style.underline(),
                        "blink" => style = style.blink(),
                        "blink_fast" => style = style.blink_fast(),
                        "reversed" => style = style.reversed(),
                        "hidden" => style = style.hidden(),
                        "strikethrough" => style = style.strikethrough(),
                        w => {
                            let p = colors_file.display();
                            bail!(
                                "Line {i} of colors config file ({p}) contains invalid style: '{w}'"
                            )
                        }
                    };
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
                    "output_create" => colors.output_create = style,
                    "output_delete" => colors.output_delete = style,
                    "output_missing" => colors.output_missing = style,
                    "output_diff" => colors.output_diff = style,
                    "diff_deleted" => colors.diff_deleted = style,
                    "diff_inserted" => colors.diff_inserted = style,
                    "diff_header" => colors.diff_header = style,
                    "show_header" => colors.show_header = style,
                    w => {
                        let p = colors_file.display();
                        bail!(
                            "Line {i} of colors config file ({p}) contains invalid element: '{w}'"
                        )
                    }
                }
            }
        }

        Ok(colors)
    }
}

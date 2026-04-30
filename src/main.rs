use std::env;

use autosaver::cli::{
    actions::Runner,
    error::Result,
    flags::ParsedArgs,
    output::{RendererOptions, TermRenderer},
};

fn main() -> Result<()> {
    let parsed_args = ParsedArgs::parse(env::args().collect());
    let renderer = TermRenderer::new(RendererOptions::new(true));
    let mut runner = Runner::new(parsed_args, renderer);
    runner.run()
}

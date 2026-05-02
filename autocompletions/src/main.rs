use clap::{Arg, Command};
use clap_complete::{generate_to, shells::Shell};
use std::path::PathBuf;

fn build_cli() -> Command {
    Command::new("autosaver")
        .version("0.3.0")
        .arg(Arg::new("nocolor").long("nocolor").global(true))
        .subcommand(
            Command::new("list")
                .about("Show differences")
                .arg(Arg::new("profile")),
        )
        .subcommand(
            Command::new("save")
                .about("Save to backup")
                .arg(Arg::new("profile"))
                .arg(Arg::new("assumeyes").short('y').long("assumeyes"))
                .arg(Arg::new("assumeno").short('n').long("assumeno"))
                .arg(Arg::new("all").short('a').long("all")),
        )
        .subcommand(
            Command::new("restore")
                .about("Restore from backup")
                .arg(Arg::new("profile"))
                .arg(Arg::new("assumeyes").short('y').long("assumeyes"))
                .arg(Arg::new("assumeno").short('n').long("assumeno"))
                .arg(Arg::new("all").short('a').long("all")),
        )
        .subcommand(
            Command::new("rmhome")
                .about("Delete from home")
                .arg(Arg::new("profile"))
                .arg(Arg::new("assumeyes").short('y').long("assumeyes"))
                .arg(Arg::new("assumeno").short('n').long("assumeno")),
        )
        .subcommand(
            Command::new("rmbackup")
                .about("Delete from backup")
                .arg(Arg::new("profile"))
                .arg(Arg::new("assumeyes").short('y').long("assumeyes"))
                .arg(Arg::new("assumeno").short('n').long("assumeno")),
        )
}

fn main() {
    let out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("completions");
    std::fs::create_dir_all(&out_dir).unwrap();

    let cmd = build_cli();

    for shell in [Shell::Bash, Shell::Zsh, Shell::Fish] {
        generate_to(shell, &mut cmd.clone(), "autosaver", &out_dir).unwrap();
    }

    println!("Completions generated in: {}", out_dir.display());
}

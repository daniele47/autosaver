use std::env;

use autosaver::core::{
    error::Result,
    fs::{AbsPath, LineWriter},
    profile::{Profile, ProfileType, composite::HashMapProfileLoader},
};

fn purge_path_even_on_panic(tmpdir: &AbsPath) -> impl Drop {
    struct Guard(AbsPath);
    impl Drop for Guard {
        fn drop(&mut self) {
            let _ = self.0.purge_path(true);
        }
    }
    Guard(tmpdir.clone())
}

fn main() -> Result<()> {
    println!("Binary version: {}", env!("CARGO_PKG_VERSION"));

    let tmpdir = AbsPath::new_tmp("rust_example");
    tmpdir.create_dir()?;
    let _guard = purge_path_even_on_panic(&tmpdir);

    // create first module
    let tmpfile1 = tmpdir.joins(&["neovim.conf"]);
    let mut writer = tmpfile1.line_writer()?;
    writer.write_all_lines([
        "/! type module",
        "",
        "// just testing with neovim configuration as an example",
        ".config/nvim",
        "",
        "/! policy ignore",
        ".config/nvim/lazy-lock.json",
    ])?;

    // create second module
    let tmpfile2 = tmpdir.joins(&["tmux.conf"]);
    let mut writer = tmpfile2.line_writer()?;
    writer.write_all_lines([
        "/! type module",
        "",
        "// just testing with neovim configuration as an example",
        ".config/tmux",
    ])?;

    // create profile with both
    let tmpfile3 = tmpdir.joins(&["tools.conf"]);
    let mut writer = tmpfile3.line_writer()?;
    writer.write_all_lines(["/! type composite", "", "neovim", "tmux"])?;

    // create profile with both
    let tmpfile4 = tmpdir.joins(&["all.conf"]);
    let mut writer = tmpfile4.line_writer()?;
    writer.write_all_lines(["/! type composite", "", "neovim", "tmux", "tools"])?;

    // load all profiles
    let mut profile_loader = HashMapProfileLoader::new();
    let profiles = profile_loader.profiles();
    for (p, f) in [
        ("neovim", &tmpfile1),
        ("tmux", &tmpfile2),
        ("tools", &tmpfile3),
        ("all", &tmpfile4),
    ] {
        profiles.insert(p.into(), Profile::parse(p.into(), f.line_reader()?)?);
    }
    let profiles = profile_loader.profiles().clone();

    for profile in profiles {
        match profile.1.ptype() {
            ProfileType::Composite(composite) => {
                let composite = composite.resolve(profile.0.as_str(), &mut profile_loader)?;
                println!("RESOLVED COMPOSITE: {composite:#?}\n\n");
            }
            ProfileType::Module(module) => {
                let module = module.resolve(&AbsPath::from(env::var("HOME").unwrap().as_str()));
                println!("RESOLVED MODULE: {module:#?}\n\n");
            }
        }
    }

    Ok(())
}

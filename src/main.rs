use autosaver::core::{
    error::Result,
    fs::{AbsPath, LineWriter},
    profile::Profile,
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

    let tmpfile = tmpdir.joins(&["neovim.conf"]);
    let mut writer = tmpfile.line_writer()?;
    writer.write_line("/! type module")?;
    writer.write_line("")?;
    writer.write_line("// just testing with neovim configuration as an example")?;
    writer.write_line(".config/nvim")?;
    writer.write_line("")?;
    writer.write_line("/! policy ignore")?;
    writer.write_line(".config/nvim/lazy-lock.json")?;
    writer.flush()?;

    let reader = tmpfile.line_reader()?;
    let profile = Profile::parse("neovim".to_string(), reader)?;

    println!("\nPARSED:\n{profile:#?}");

    match profile.ptype() {
        autosaver::core::profile::ProfileType::Composite(_composite) => todo!(),
        autosaver::core::profile::ProfileType::Module(module) => {
            let resolved_profile = module.resolve(&AbsPath::from(env!("HOME")))?;
            println!("\nRESOLVED:\n{resolved_profile:#?}");
        }
    }

    Ok(())
}

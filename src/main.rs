use dotfiles_rust::core::{
    errors::{Error, Result},
    fs::AbsPath,
};

fn main() -> Result<()> {
    let abs = AbsPath::new_tmp("read_lines_example");
    abs.create_file(true).unwrap();
    for line in abs.read_lines().unwrap() {
        let _ = line
            .map_err(|e| Error::IoError(e, abs.clone().into()))
            .unwrap();
        // ops on line here
    }
    abs.purge_path(true).unwrap();
    Ok(())
}

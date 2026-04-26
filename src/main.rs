use autosaver::core::{errors::Result, fs::AbsPath};

fn main() -> Result<()> {
    let abs = AbsPath::from("/etc/passwd");
    abs.create_file(true)?;
    for line in abs.read_lines()? {
        let line = line?;
        println!("{line}");
    }
    Ok(())
}

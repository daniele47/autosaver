use autosaver::core::{
    error::Result,
    fs::{AbsPath, LineWriter},
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

    let tmpfile = tmpdir.joins(&["tmpfile.txt"]);
    let mut writer = tmpfile.write_lines()?;
    writer.write_line("Line 1")?;
    writer.write_line("Line 2")?;
    writer.write_line("It's the final line!!!")?;
    writer.flush()?;

    println!("\nReading {}:", tmpfile.to_str_lossy());
    for line in tmpfile.read_lines()? {
        println!("{}", line?);
    }

    Ok(())
}

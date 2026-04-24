use dotfiles_rust::core::fs::AbsPath;

fn main() {
    let abs = AbsPath::from("/tmp/testing");
    println!("creatig dir: {:?}", abs);
    abs.create_dir().unwrap();
}

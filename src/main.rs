use std::str::FromStr;

use anyhow::Result;

use crate::fs::abs::AbsPathStr;

pub mod fs;
pub mod prof;

fn main() -> Result<()> {
    let path = AbsPathStr::from_str("/bin/tree")?;
    let base = AbsPathStr::from_str("/bin")?;
    let rel = path.to_rel(&base)?;
    println!("{rel:?}");
    let abs = rel.to_abs(&base)?;
    println!("{abs:?}");

    Ok(())
}

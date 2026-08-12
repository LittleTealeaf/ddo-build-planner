use std::{env, fs::File, io::Write, path::Path};

use anyhow::Result;
use serde::Serialize;

fn main() -> Result<()> {
    write_artifact("test", vec!["hello", "world"])?;
    Ok(())
}

fn write_artifact<P, S>(name: P, item: S) -> Result<()>
where
    P: AsRef<Path>,
    S: Serialize,
{
    let path = Path::new(&env::var("OUT_DIR")?).join(name);
    let mut file = File::create(path)?;
    let serialized = ron::to_string(&item)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}

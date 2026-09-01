use std::{
    env,
    fs::File,
    io::{BufReader, Write},
    path::Path,
};

use anyhow::Result;
use ddo_core::items::feat::Feat;
use ron::de::from_reader;
use serde::Serialize;

fn main() -> Result<()> {
    write_artifact("test", vec!["hello", "world"])?;
    write_artifact("feats", read_artifact::<Vec<Feat>, _>("feats.ron")?)?;
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

fn read_artifact<S, P>(name: P) -> Result<S>
where
    P: AsRef<Path>,
    S: for<'de> serde::Deserialize<'de>,
{
    let path = Path::new("./data").join(name);
    println!(
        "cargo:rerun-if-changed={}",
        path.to_str().ok_or_else(|| anyhow::anyhow!("Error"))?
    );
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let items = from_reader(reader)?;
    Ok(items)
}

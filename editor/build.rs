use std::{env, fs::File, io::Write, path::Path};

fn main() -> Result<(), BuildErrors> {
    let out_path = Path::new(&env::var("OUT_DIR")?).join("repo_path");
    let manifest_dir_env = env::var("CARGO_MANIFEST_DIR")?;
    let repo_path = Path::new(&manifest_dir_env)
        .parent()
        .ok_or(BuildErrors::NoManifestParent)?;

    let mut out_file = File::create(out_path)?;

    out_file.write_all(
        repo_path
            .to_str()
            .ok_or(BuildErrors::InvalidRepoPath)?
            .as_bytes(),
    )?;

    Ok(())
}

#[derive(Debug)]
enum BuildErrors {
    IO(std::io::Error),
    Environment(env::VarError),
    NoManifestParent,
    InvalidRepoPath,
}

impl From<env::VarError> for BuildErrors {
    fn from(value: env::VarError) -> Self {
        Self::Environment(value)
    }
}

impl From<std::io::Error> for BuildErrors {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

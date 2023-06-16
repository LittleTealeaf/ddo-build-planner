use std::env;

fn main() -> Result<(), self::errors::Error> {
    let _out_dir = env::var("OUT_DIR")?;

    Ok(())
}

mod errors {
    use std::env::VarError;

    #[derive(Debug)]
    pub enum Error {
        VarError(VarError),
    }

    impl From<VarError> for Error {
        fn from(value: VarError) -> Self {
            Self::VarError(value)
        }
    }
}

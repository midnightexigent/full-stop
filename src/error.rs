use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IntoUtf8PathBuf(#[from] camino::FromPathBufError),
    #[error(transparent)]
    RelativePath(#[from] relative_path::FromPathError),
    #[error(transparent)]
    Ignore(#[from] ignore::Error),
    #[error(transparent)]
    GlobPattern(#[from] glob::PatternError),
    #[error(transparent)]
    Glob(#[from] glob::GlobError),
    #[error(transparent)]
    ShellExpandLookup(#[from] shellexpand::LookupError<Box<Error>>),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    SerializeToml(#[from] toml::ser::Error),
    #[error(transparent)]
    DeserializeToml(#[from] toml::de::Error),
}

impl From<shellexpand::LookupError<Error>> for Error {
    fn from(error: shellexpand::LookupError<Error>) -> Self {
        Self::ShellExpandLookup(shellexpand::LookupError {
            var_name: error.var_name,
            cause: Box::new(error.cause),
        })
    }
}

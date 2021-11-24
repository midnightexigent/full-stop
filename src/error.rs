#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ReadConfig(#[from] ReadConfig),
    #[error(transparent)]
    RelativePathError(#[from] relative_path::FromPathError),
    #[error(transparent)]
    ReadConfigFile(#[from] std::io::Error),
    #[error(transparent)]
    GlobSet(#[from] globset::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum ReadConfig {
    #[error("Invalid module name: '{0}'")]
    InvalidModuleName(String),
    #[error(transparent)]
    ParseToml(#[from] toml::de::Error),
    #[error(transparent)]
    OpenFile(#[from] std::io::Error),
}

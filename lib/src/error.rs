use thiserror::Error;
#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("serializing a type failed: {0}")]
    TomlSerError(#[from] toml::ser::Error),
    #[error("from_str error: {0}")]
    FromStrError(String),
    #[error("root node must be Struct type")]
    RootMustStruct,
    #[error("enum no variant")]
    EnumEmpty,
}

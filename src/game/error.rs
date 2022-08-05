use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum InitError {
    #[error("no window")]
    NoWindow,
    #[error("no document")]
    NoDocument,
    #[error("element creation failed\n{0}")]
    ElementCreationFailed(String),
    #[error("no container")]
    NoContainer,
    #[error("failed to append child\n{0}")]
    AppendChildFailed(String),
}

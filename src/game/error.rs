use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum InitError {
    #[error("no window")]
    NoWindow,
    #[error("no document")]
    NoDocument,
    #[error("canvas creation failed\n{0}")]
    CanvasCreationFailed(String),
    #[error("no container")]
    NoContainer,
    #[error("failed to append child\n{0}")]
    AppendChildFailed(String),
}

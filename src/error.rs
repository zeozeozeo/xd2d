use thiserror::Error;

#[derive(Debug, Error)]
pub enum XdError {
    #[error("glutin GL context creation error: {0}")]
    CreationError(#[from] glutin::CreationError),
    #[error("GL context error: {0}")]
    ContextError(#[from] glutin::ContextError),
}

pub type XdResult<T> = Result<T, XdError>;

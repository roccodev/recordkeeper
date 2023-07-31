use std::convert::Infallible;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StructError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("unreachable")]
    Infallible(#[from] Infallible),
}

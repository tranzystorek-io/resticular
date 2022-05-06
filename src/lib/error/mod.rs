use thiserror::Error;
use crossbeam_channel::{RecvError, SendError};
#[derive(Debug, Error)]
pub enum Error {
    #[error("Parser Error: {0}")]
    ParserError(String),
    #[error("File I/O: {0}")]
    FileIOError(#[from] std::io::Error),
    #[error("{0}")]
    HtmlMinifyError(String),
    #[error("Condition failed")]
    PageCheckError,
    #[error("Html rewriting error {0}")]
    HtmlRewriteError(#[from] lol_html::errors::RewritingError)
}

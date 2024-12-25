use thiserror::Error;

use super::Rule;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Syntax error: {0}")]
    SyntaxError(String),
    #[error("Unknown error: {0}")]
    UnknownError(String),
    #[error("Pest Parser error: {0}")]
    PestError(#[from] pest::error::Error<Rule>),
}

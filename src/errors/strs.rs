use super::{CausableConsumeError, TokenConsumeError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum StringConsumeError {
    #[error("Expected more tokens, but none were found!")]
    InsufficientTokens,

    #[error("Unexpected token `{token}` at index `{index}`")]
    UnexpectedToken { index: usize, token: char },
}

impl CausableConsumeError for StringConsumeError {
    fn ref_causes(&self) -> Vec<(&usize, &char)> {
        use StringConsumeError::*;

        match self {
            InsufficientTokens => Vec::new(),
            UnexpectedToken { index, token } => vec![(index, token)],
        }
    }

    fn mut_causes(&mut self) -> Vec<(&mut usize, &mut char)> {
        use StringConsumeError::*;

        match self {
            InsufficientTokens => Vec::new(),
            UnexpectedToken {
                ref mut index,
                ref mut token,
            } => vec![(index, token)],
        }
    }
}

impl From<TokenConsumeError> for StringConsumeError {
    fn from(from: TokenConsumeError) -> Self {
        match from {
            TokenConsumeError::EmptyString => StringConsumeError::InsufficientTokens,
            TokenConsumeError::UnexpectedToken { index, token } => {
                StringConsumeError::UnexpectedToken { index, token }
            }
        }
    }
}

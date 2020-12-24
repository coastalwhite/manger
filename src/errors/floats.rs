use super::{CausableConsumeError, StringConsumeError, TokenConsumeError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum FloatConsumeError {
    #[error("Number that needed to consumed was too big for current datatype")]
    FloatError(#[from] std::num::ParseFloatError),

    #[error("Invalid token `{token}` at index `{index}`")]
    UnexpectedToken { index: usize, token: char },

    #[error("Expected more tokens, but none were found!")]
    InsufficientTokens,
}

impl CausableConsumeError for FloatConsumeError {
    fn ref_causes(&self) -> Vec<(&usize, &char)> {
        use FloatConsumeError::*;

        match self {
            FloatError(_) | InsufficientTokens => Vec::new(),
            UnexpectedToken { index, token } => vec![(index, token)],
        }
    }

    fn mut_causes(&mut self) -> Vec<(&mut usize, &mut char)> {
        use FloatConsumeError::*;

        match self {
            FloatError(_) | InsufficientTokens => Vec::new(),
            UnexpectedToken {
                ref mut index,
                ref mut token,
            } => vec![(index, token)],
        }
    }
}

impl From<TokenConsumeError> for FloatConsumeError {
    fn from(from: TokenConsumeError) -> Self {
        match from {
            TokenConsumeError::EmptyString => FloatConsumeError::InsufficientTokens,
            TokenConsumeError::UnexpectedToken { index, token } => {
                FloatConsumeError::UnexpectedToken { index, token }
            }
        }
    }
}

impl From<StringConsumeError> for FloatConsumeError {
    fn from(from: StringConsumeError) -> Self {
        match from {
            StringConsumeError::InsufficientTokens => FloatConsumeError::InsufficientTokens,
            StringConsumeError::UnexpectedToken { index, token } => {
                FloatConsumeError::UnexpectedToken { index, token }
            }
        }
    }
}

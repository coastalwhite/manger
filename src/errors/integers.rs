use super::{CausableConsumeError, StringConsumeError, TokenConsumeError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum IntegerConsumeError {
    #[error("Number that needed to consumed was too big for current datatype")]
    Overflow,

    #[error("Invalid token `{token}` at index `{index}`")]
    UnexpectedToken { index: usize, token: char },

    #[error("Expected more tokens, but none were found!")]
    InsufficientTokens,
}

impl CausableConsumeError for IntegerConsumeError {
    fn ref_causes(&self) -> Vec<(&usize, &char)> {
        use IntegerConsumeError::*;

        match self {
            Overflow | InsufficientTokens => Vec::new(),
            UnexpectedToken { index, token } => vec![(index, token)],
        }
    }

    fn mut_causes(&mut self) -> Vec<(&mut usize, &mut char)> {
        use IntegerConsumeError::*;

        match self {
            Overflow | InsufficientTokens => Vec::new(),
            UnexpectedToken {
                ref mut index,
                ref mut token,
            } => vec![(index, token)],
        }
    }
}

impl From<TokenConsumeError> for IntegerConsumeError {
    fn from(from: TokenConsumeError) -> Self {
        match from {
            TokenConsumeError::EmptyString => IntegerConsumeError::InsufficientTokens,
            TokenConsumeError::UnexpectedToken { index, token } => {
                IntegerConsumeError::UnexpectedToken { index, token }
            }
        }
    }
}

impl From<StringConsumeError> for IntegerConsumeError {
    fn from(from: StringConsumeError) -> Self {
        match from {
            StringConsumeError::InsufficientTokens => IntegerConsumeError::InsufficientTokens,
            StringConsumeError::UnexpectedToken { index, token } => {
                IntegerConsumeError::UnexpectedToken { index, token }
            }
        }
    }
}

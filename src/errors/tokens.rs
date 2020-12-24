use super::CausableConsumeError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum TokenConsumeError {
    #[error("Tried to consume character from an empty string.")]
    EmptyString,

    #[error("Invalid token `{token}` was found at index {index}.")]
    UnexpectedToken { index: usize, token: char },
}

impl CausableConsumeError for TokenConsumeError {
    fn ref_causes(&self) -> Vec<(&usize, &char)> {
        use TokenConsumeError::*;

        match self {
            EmptyString => Vec::new(),
            UnexpectedToken { index, token } => vec![(index, token)],
        }
    }

    fn mut_causes(&mut self) -> Vec<(&mut usize, &mut char)> {
        use TokenConsumeError::*;

        match self {
            EmptyString => Vec::new(),
            UnexpectedToken {
                ref mut index,
                ref mut token,
            } => vec![(index, token)],
        }
    }
}

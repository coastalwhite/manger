use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone, Copy)]
pub enum ConsumeErrorType {
    #[error("Expected more tokens at index `{index}` but found none!")]
    InsufficientTokens { index: usize },

    #[error("Found the token `{token}` at index `{index}`, which is unexpected!")]
    UnexpectedToken { index: usize, token: char },

    #[error("Tried to form a value which was not allowed at index `{index}`. Maybe there was an overflow?")]
    InvalidValue { index: usize },
}

#[derive(Debug, PartialEq)]
pub struct ConsumeError {
    causes: Vec<ConsumeErrorType>,
}

impl ConsumeError {
    pub fn new() -> ConsumeError {
        ConsumeError { causes: Vec::new() }
    }

    pub fn new_with(cause: ConsumeErrorType) -> ConsumeError {
        ConsumeError {
            causes: vec![cause],
        }
    }

    pub fn new_from(causes: Vec<ConsumeErrorType>) -> ConsumeError {
        ConsumeError { causes }
    }

    pub fn offset(mut self, by: usize) -> Self {
        self.causes
            .iter_mut()
            .for_each(|cause| *cause = cause.offset(by));
        self
    }

    pub fn causes(&self) -> &Vec<ConsumeErrorType> {
        &self.causes
    }

    pub fn add_cause(&mut self, cause: ConsumeErrorType) {
        self.causes.push(cause);
    }

    pub fn add_causes(&mut self, error: ConsumeError) {
        error
            .causes()
            .iter()
            .for_each(|cause| self.add_cause(*cause));
    }
}

impl ConsumeErrorType {
    pub fn index(&self) -> usize {
        use ConsumeErrorType::*;

        match self {
            InsufficientTokens { index } => *index,
            UnexpectedToken { index, token: _ } => *index,
            InvalidValue { index } => *index,
        }
    }

    pub fn offset(self, by: usize) -> Self {
        use ConsumeErrorType::*;

        match self {
            InsufficientTokens { index } => InsufficientTokens { index: index + by },
            UnexpectedToken { index, token } => UnexpectedToken {
                index: index + by,
                token,
            },
            InvalidValue { index } => InvalidValue { index: index + by },
        }
    }
}

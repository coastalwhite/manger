use thiserror::Error;

/// One item in [`ConsumeError`]. these can occur while consuming
/// from a `source.
///
/// Multiple instances of this type can occur during one parsing.
/// Especially, multiple instance of these error occur,
/// when using `enum`'s or using the `Either<L, R>` struct.
#[derive(Error, Debug, PartialEq, Clone, Copy)]
pub enum ConsumeErrorType {
    /// An error varient which occurs when while consuming more tokens
    /// where expected, but none were found.
    #[error("Expected more tokens at index `{index}` but found none!")]
    InsufficientTokens {
        /// The utf-8 character index within the `source` at which more tokens were expected, but not
        /// found.
        index: usize,
    },

    /// An error varient which occurs when while consuming a token that was not expected is
    /// presented.
    #[error("Found the token `{token}` at index `{index}`, which is unexpected!")]
    UnexpectedToken {
        /// The utf-8 character index within the `source` at which an unexpected token was found.
        index: usize,
        /// The utf-8 character which was unexpected.
        token: char,
    },

    /// An error varient which occurs when while consuming a consume condition is not met.
    ///
    /// This happens most often when a condition is specified for consumation, but it is not met.
    /// However, this also happens when a integer or float overflows tries to assume an incorrect
    /// value.
    #[error("Tried to form a value which was not allowed at index `{index}`. Maybe there was an overflow?")]
    InvalidValue {
        /// The utf-8 character index within the `source` at which an invalid value started to be
        /// formed.
        index: usize,
    },
}

/// A list of errors that occured while consuming from a `source`.
#[derive(Debug, PartialEq)]
pub struct ConsumeError {
    causes: Vec<ConsumeErrorType>,
}

impl ConsumeError {
    /// Create a new empty `ConsumeError`.
    pub fn new() -> ConsumeError {
        ConsumeError { causes: Vec::new() }
    }

    /// Create a new `ConsumeError` containing only `cause`.
    pub fn new_with(cause: ConsumeErrorType) -> ConsumeError {
        ConsumeError {
            causes: vec![cause],
        }
    }

    /// Create a new `ConsumeError` containing `causes`.
    pub fn new_from(causes: Vec<ConsumeErrorType>) -> ConsumeError {
        ConsumeError { causes }
    }

    /// Mutate all the errors to move the utf-8 character index at which they were caused by `by`.
    ///
    /// # Examples
    ///
    /// ```
    /// use manger::{ ConsumeError, ConsumeErrorType::* };
    /// assert_eq!(
    ///     ConsumeError::new_from(
    ///         vec![
    ///             InvalidValue { index: 0 },
    ///             InsufficientTokens { index: 5 }
    ///         ]
    ///     ).offset(2),
    ///     ConsumeErrorType::new_from(
    ///         vec![
    ///             InvalidValue { index: 2 },
    ///             InsufficientTokens { index: 7 }
    ///         ]
    ///     )
    /// );
    /// ```
    pub fn offset(mut self, by: usize) -> Self {
        self.causes
            .iter_mut()
            .for_each(|cause| *cause = cause.offset(by));
        self
    }

    /// Fetch a vector of the causes of this error.
    ///
    /// This consume ownership of the error.
    pub fn into_causes(self) -> Vec<ConsumeErrorType> {
        self.causes
    }

    /// Fetch a vector of references to the causes of this error.
    pub fn causes(&self) -> Vec<&ConsumeErrorType> {
        self.causes.iter().collect()
    }

    /// Pushes an extra cause for this error.
    pub fn add_cause(&mut self, cause: ConsumeErrorType) {
        self.causes.push(cause);
    }

    /// Pushes all the causes for `other_err` for this error.
    pub fn add_causes(&mut self, other_err: ConsumeError) {
        other_err
            .into_causes()
            .into_iter()
            .for_each(|cause| self.add_cause(cause));
    }
}

impl ConsumeErrorType {
    /// Fetch the utf-8 character index at which a consume error occured.
    pub fn index(&self) -> &usize {
        use ConsumeErrorType::*;

        match self {
            InsufficientTokens { index } => index,
            UnexpectedToken { index, token: _ } => index,
            InvalidValue { index } => index,
        }
    }

    /// Mutate self to move the utf-8 character index at which they were caused by `by`.
    ///
    /// # Examples
    ///
    /// ```
    /// use manger::ConsumeErrorType::*;
    /// assert_eq!(
    ///     InvalidValue { index: 0 }.offset(2),
    ///     InvalidValue { index: 2 },
    /// );
    /// ```
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

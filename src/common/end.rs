use crate::Consumable;
use crate::{ConsumeError, ConsumeErrorType};

/// End of stream of tokens.
///
/// Will succeed in consumation if the end of string has been reached. Will fail if it has not been
/// reached.
///
/// # Examples
///
/// ```
/// use manger::{consume_struct, Consumable};
/// use manger::common;
///
/// #[derive(PartialEq, Debug)]
/// struct EncasedInteger(i32);
/// consume_struct!(
///     EncasedInteger => [
///         > '(',
///         value: i32,
///         > ')',
///         : common::End;
///         (value)
///     ]
/// );
///
/// assert!(EncasedInteger::consume_from("(42)").is_ok());
/// assert!(EncasedInteger::consume_from("(42) some leftover tokens").is_err());
/// ```
#[derive(Debug, PartialEq)]
pub struct End;

impl Consumable for End {
    fn consume_from(source: &str) -> Result<(Self, &str), ConsumeError> {
        if source.is_empty() {
            Ok((End, ""))
        } else {
            Err(ConsumeError::new_with(ConsumeErrorType::UnexpectedToken {
                index: 0,
                token: source.chars().next().unwrap(),
            }))
        }
    }
}

use crate::chars;
use crate::error::ConsumeError;
use crate::Consumable;
use either::Either;

#[derive(Debug, PartialEq)]
pub struct Sign(pub bool);

impl Consumable for Sign {
    fn consume_from(s: &str) -> Result<(Self, &str), ConsumeError> {
        let (item, unconsumed) = <Either<chars::Hyphen, Option<chars::Plus>>>::consume_from(s)?;
        Ok((Sign(item.is_right()), unconsumed))
    }
}

#[derive(Debug, PartialEq)]
pub struct Empty;

impl Consumable for Empty {
    fn consume_from(s: &str) -> Result<(Self, &str), ConsumeError> {
        Ok((Empty, s))
    }
}

#[derive(Debug, PartialEq)]
pub struct Digit(pub u8);

impl Consumable for Digit {
    fn consume_from(s: &str) -> Result<(Self, &str), ConsumeError> {
        use crate::error::ConsumeErrorType::*;

        if let Some(token) = s.chars().next() {
            Ok((
                Digit(
                    token
                        .to_digit(10)
                        .ok_or(ConsumeError::new_with(UnexpectedToken { index: 0, token }))?
                        as u8,
                ),
                utf8_slice::from(s, 1),
            ))
        } else {
            Err(ConsumeError::new_with(InsufficientTokens { index: 0 }))
        }
    }
}

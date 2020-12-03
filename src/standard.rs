use crate::chars;
use crate::chars::CharConsumeError;
use crate::Consumable;
use either::Either;

#[derive(Debug, PartialEq)]
pub struct Sign(pub bool);

impl Consumable for Sign {
    type ConsumeError = CharConsumeError;

    fn consume_from(s: &str) -> Result<(Self, &str), Self::ConsumeError> {
        let (item, unconsumed) = <Either<chars::Hyphen, Option<chars::Plus>>>::consume_from(s)?;
        Ok((Sign(item.is_right()), unconsumed))
    }
}

#[derive(Debug, PartialEq)]
pub struct Empty;

impl Consumable for Empty {
    type ConsumeError = ();

    fn consume_from(s: &str) -> Result<(Self, &str), Self::ConsumeError> {
        Ok((Empty, s))
    }
}

#[derive(Debug, PartialEq)]
pub struct Digit(pub u8);

impl Consumable for Digit {
    type ConsumeError = CharConsumeError;

    fn consume_from(s: &str) -> Result<(Self, &str), Self::ConsumeError> {
        if let Some(c) = s.chars().next() {
            Ok((
                Digit(c.to_digit(10).ok_or(CharConsumeError::InvalidToken(0, c))? as u8),
                utf8_slice::from(s, 1),
            ))
        } else {
            Err(CharConsumeError::EmptyString)
        }
    }
}

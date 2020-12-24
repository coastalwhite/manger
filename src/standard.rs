use crate::chars;
use crate::errors::TokenConsumeError;
use crate::Consumable;
use either::Either;

#[derive(Debug, PartialEq)]
pub struct Sign(pub bool);

impl Consumable for Sign {
    type ConsumeError = (TokenConsumeError, TokenConsumeError);

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
    type ConsumeError = TokenConsumeError;

    fn consume_from(s: &str) -> Result<(Self, &str), Self::ConsumeError> {
        if let Some(token) = s.chars().next() {
            Ok((
                Digit(
                    token
                        .to_digit(10)
                        .ok_or(TokenConsumeError::UnexpectedToken { index: 0, token })?
                        as u8,
                ),
                utf8_slice::from(s, 1),
            ))
        } else {
            Err(TokenConsumeError::EmptyString)
        }
    }
}

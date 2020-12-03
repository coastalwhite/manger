use ::either::{Either, Either::Left, Either::Right};

pub type OneOrMore<T> = (T, Vec<T>);
pub type MultipleWithDelimiter<T, D> = (Vec<(T, D)>, T);

/// Trait used to do efficient parsing.
pub trait Consumable: Sized {
    type ConsumeError;

    fn consume(s: &str) -> Result<(Self, &str), Self::ConsumeError>;
    fn try_consume(s: &str) -> (Option<Self>, &str) {
        let result = Self::consume(s);

        match result {
            Ok((item, unconsumed)) => (Some(item), unconsumed),
            Err(_) => (None, s),
        }
    }
}

pub trait ASCIIConsumable: Sized {
    type ASCIIConsumeError;

    fn ascii_consume_from(s: &str) -> Result<(Self, &str), Self::ASCIIConsumeError>;
    fn try_ascii_consume_from(s: &str) -> Result<(Self, &str), Self::ASCIIConsumeError> {
        let result = Self::ascii_consume_from(s);

        match result {
            Ok((item, unconsumed)) => (Some(item), unconsumed),
            Err(_) => (None, s),
        }
    }
}

impl<T: ASCIIConsumable> Consumable for T {
    type ConsumeError = Self::ASCIIConsumeError;

    fn consume(s: &str) -> Result<(Self, &str), Self::ConsumeError> {
        Self::ascii_consume_from(s)
    }
}

impl<T: Consumable> Consumable for Option<T> {
    type ConsumeError = T::ConsumeError;

    fn consume(s: &str) -> Result<(Self, &str), Self::ConsumeError> {
        Ok(T::try_consume(s))
    }
}

impl<T: Consumable> Consumable for Vec<T> {
    type ConsumeError = T::ConsumeError;

    fn consume(s: &str) -> Result<(Self, &str), Self::ConsumeError> {
        let mut sequence = Vec::new();
        let mut last_unconsumed = s;

        while let Ok((extra_coordinate_pair, unconsumed)) = T::consume(last_unconsumed) {
            sequence.push(extra_coordinate_pair);
            last_unconsumed = unconsumed;
        }

        Ok((sequence, last_unconsumed))
    }
}

impl<T, J> Consumable for (T, J)
where
    T: Consumable,
    J: Consumable,
{
    type ConsumeError = Either<T::ConsumeError, J::ConsumeError>;

    fn consume(s: &str) -> Result<(Self, &str), Self::ConsumeError> {
        let (t_item, unconsumed) = T::consume(s).map_err(|err| Left(err))?;
        let (j_item, unconsumed) = J::consume(unconsumed).map_err(|err| Right(err))?;

        Ok(((t_item, j_item), unconsumed))
    }
}

pub mod bytes;
pub mod chars;
mod either;
pub mod floats;
pub mod integers;
pub mod standard;
pub mod util;

use ::either::{Either, Either::Left, Either::Right};

pub type OneOrMore<T> = (T, Vec<T>);
pub type MultipleWithDelimiter<T, D> = (Vec<(T, D)>, T);

/// Trait used to do efficient parsing.
pub trait ConsumeParsable: Sized {
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

impl<T: ConsumeParsable> ConsumeParsable for Option<T> {
    type ConsumeError = T::ConsumeError;

    fn consume(s: &str) -> Result<(Self, &str), Self::ConsumeError> {
        Ok(T::try_consume(s))
    }
}

impl<T: ConsumeParsable> ConsumeParsable for Vec<T> {
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

impl<T, J> ConsumeParsable for (T, J)
where
    T: ConsumeParsable,
    J: ConsumeParsable,
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

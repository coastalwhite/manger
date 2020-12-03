use ::either::{Either, Either::Left, Either::Right};

pub type OneOrMore<T> = (T, Vec<T>);
pub type MultipleWithDelimiter<T, D> = (Vec<(T, D)>, T);

/// Trait used to do efficient parsing.
pub trait Consumable: Sized {
    type ConsumeError;

    fn consume_from(s: &str) -> Result<(Self, &str), Self::ConsumeError>;
    fn try_consume_from(s: &str) -> (Option<Self>, &str) {
        let result = Self::consume_from(s);

        match result {
            Ok((item, unconsumed)) => (Some(item), unconsumed),
            Err(_) => (None, s),
        }
    }
    fn consume_till_error_from(s: &str) -> (Vec<Self>, &str) {
        <Vec<Self>>::consume_from(s).unwrap_or((Vec::new(), s))
    }
    fn consume_till_end_from(s: &str) -> Result<Vec<Self>, Either<Self::ConsumeError, &str>> {
        let (vs, unconsumed) = <Vec<Self>>::consume_from(s).map_err(|err| Left(err))?;

        if unconsumed.is_empty() {
            Ok(vs)
        } else {
            Err(Right(unconsumed))
        }
    }
}

pub struct ConsumeIter<'a, T>
where
    T: Consumable,
{
    phantom: std::marker::PhantomData<T>,
    unconsumed: &'a str,
}

impl<'a, T> Iterator for ConsumeIter<'a, T>
where
    T: Consumable,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        <T>::try_consume_from(self.unconsumed).0
    }
}

pub mod chars;
mod either;
pub mod floats;
mod impls;
pub mod integers;
pub mod standard;
pub mod util;

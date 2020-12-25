//! # Manger
//!
//! ## A performant, low-level, lightweight and intuitive parsing library
//!
//! ## Why use Manger
//!
//! Manger is really easy to use, easy to understand and really performant.
//! It is inspired by combinatoric parsers.
//! Manger has a optimatized standard library including parsing for integers,
//! floating-point and UTF-8.

use ::either::{Either, Either::Left, Either::Right};
use errors::CausableConsumeError;

/// Consume one or more of type _T_.
/// This is equalent of the `+` operator in EBNF syntax or within RegEx.
pub struct OneOrMore<T: Consumable> {
    /// The element that is guarenteed to be consumed
    pub head: T,
    /// Other items had are possibly parsed
    pub tail: Vec<T>,
}

impl<T: Consumable> Consumable for OneOrMore<T> {
    type ConsumeError = T::ConsumeError;

    fn consume_from(s: &str) -> Result<(Self, &str), Self::ConsumeError> {
        let (head, unconsumed) = T::consume_from(s)?;
        let (tail, unconsumed) = <Vec<T>>::consume_from(unconsumed)?;

        Ok((OneOrMore { head, tail }, unconsumed))
    }
}
/// Consume one or more with a delimiter between elements
pub type MultipleWithDelimiter<T, D> = (Vec<(T, D)>, T);

/// Trait used to do efficient parsing.
pub trait Consumable: Sized {
    type ConsumeError: CausableConsumeError;

    /// Consume part of string to form an item of Self.
    fn consume_from(s: &str) -> Result<(Self, &str), Self::ConsumeError>;

    fn consume_chars_moved(s: &str) -> Result<(Self, &str, usize), Self::ConsumeError> {
        let start_len = utf8_slice::len(s);
        let (item, unconsumed) = Self::consume_from(s)?;
        let end_len = utf8_slice::len(unconsumed);

        Ok((item, unconsumed, start_len - end_len))
    }

    /// Attempt to consume_form if that fails, return `None` and inputted string.
    ///
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

pub trait SelfConsumable {
    type ConsumeError;

    fn consume_item<'a, 'b>(
        item: &'a Self,
        s: &'b str,
    ) -> Result<(&'a Self, &'b str), Self::ConsumeError>;
}

pub trait ConsumeSource: Sized {
    fn consume<'a, 'b, T: SelfConsumable>(
        self: Self,
        item: &'b T,
    ) -> Result<(&'b T, Self), T::ConsumeError>;
}

impl<'a> ConsumeSource for &'a str {
    fn consume<'b, T: SelfConsumable>(
        self: Self,
        item: &'b T,
    ) -> Result<(&'b T, Self), T::ConsumeError> {
        <T>::consume_item(item, self)
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
pub mod errors;
pub mod floats;
mod impls;
pub mod integers;
pub mod macros;
pub mod standard;
mod strs;
pub mod util;

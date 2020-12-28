#![warn(
    future_incompatible,
    rust_2018_idioms,
    missing_docs,
    missing_doc_code_examples,
    missing_debug_implementations
)]
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

#[doc(inline)]
pub use one_or_more::OneOrMore;

#[doc(inline)]
pub use error::{ConsumeError, ConsumeErrorType};

#[doc(inline)]
pub use consumable::Consumable;

/// Consume one or more with a delimiter between elements
pub type MultipleWithDelimiter<T, D> = (Vec<(T, D)>, T);

/// Trait which allows for consuming of instances and literals from a string.
/// This trait should be mostly used for types with a bijection to a string representation,
/// which includes the `char` and `&str`. This does not include floating points, because
/// "42" and "4.2e1" will both consume to 42.
///
/// # Note
///
/// For the reason mentioned before, this is not implemented for `f32` and `f64`. Similarly,
/// this is also not implemented for `u8`, `u16`, `u32`, `u64`, `i8`, `i1
pub trait SelfConsumable {
    /// Attempt to consume a literal `item` from a `source` string. When consuming
    /// is succesful, it will return the unconsumed part of the `source`. When consuming
    /// fails, it will return an error.
    ///
    /// This is the core function implement when implementing [`SelfConsumable`](#).
    ///
    /// # Implemention note
    ///
    /// It is highly recommended to take into account UTF-8 characters. This is
    /// reasonably easy with `.chars()` on `&str` or with the crate
    /// [`utf8-slice`](https://crates.io/crates/utf8_slice).
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), manger::error::ConsumeError> {
    /// use manger::{ Consumable, SelfConsumable };
    ///
    /// let source = "scalar*42";
    ///
    /// let unconsumed = <&str>::consume_item(source, &"scalar")?;
    /// assert_eq!(unconsumed, "*42");
    ///
    /// let unconsumed = char::consume_item(unconsumed, &'*')?;
    /// assert_eq!(unconsumed, "42");
    ///
    /// let (num, unconsumed) = u32::consume_from(unconsumed)?;
    /// assert_eq!(num, 42);
    /// assert_eq!(unconsumed, "");
    /// # Ok(())
    /// # }
    /// ```
    fn consume_item<'a>(source: &'a str, item: &'_ Self) -> Result<&'a str, ConsumeError>;
}

pub trait ConsumeSource: Sized {
    /// A shorthand for the [`consume_item`](trait.SelfConsumable.html#tymethod.consume_item).
    /// Here the `source` is `self` and the `item` is `literal`.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), manger::error::ConsumeError> {
    /// use manger::ConsumeSource;
    ///
    /// let source = "{42}";
    ///
    /// let unconsumed = source.consume_lit(&'{')?;
    /// assert_eq!(unconsumed, "42}");
    ///
    /// let (num, unconsumed) = unconsumed.consume::<u32>()?;
    /// assert_eq!(num, 42);
    /// assert_eq!(unconsumed, "}");
    ///
    /// let unconsumed = unconsumed.consume_lit(&'}')?;
    /// assert_eq!(unconsumed, "");
    /// # Ok(())
    /// # }
    /// ```
    fn consume_lit<T: SelfConsumable>(self, literal: &T) -> Result<Self, ConsumeError>;

    /// A shorthand for the [`consume_from`](trait.Consumable.html#tymethod.consume_from).
    /// Here the `source` is `self`. Returns how many utf-8 characters where consumed, when
    /// succesful.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), manger::error::ConsumeError> {
    /// use manger::ConsumeSource;
    ///
    /// let source = "{42}";
    ///
    /// let unconsumed = source.consume_lit(&'{')?;
    /// assert_eq!(unconsumed, "42}");
    ///
    /// let (num, unconsumed) = unconsumed.consume::<u32>()?;
    /// assert_eq!(num, 42);
    /// assert_eq!(unconsumed, "}");
    ///
    /// let unconsumed = unconsumed.consume_lit(&'}')?;
    /// assert_eq!(unconsumed, "");
    /// # Ok(())
    /// # }
    /// ```
    fn consume<T: Consumable>(self) -> Result<(T, Self), ConsumeError>;

    /// A shorthand for the [`consume_item`](trait.SelfConsumable.html#tymethod.consume_item).
    /// Here the `source` is `self` and the `item` is `literal`.
    ///
    /// Will mutate `source` to have the unconsumed part.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), manger::error::ConsumeError> {
    /// use manger::ConsumeSource;
    ///
    /// let mut source = "{42}";
    ///
    /// source.mut_consume_lit(&'{')?;
    /// assert_eq!(source, "42}");
    ///
    /// let num = source.mut_consume::<u32>()?;
    /// assert_eq!(num, 42);
    /// assert_eq!(source, "}");
    ///
    /// source.mut_consume_lit(&'}')?;
    /// assert_eq!(source, "");
    /// # Ok(())
    /// # }
    /// ```
    fn mut_consume_lit<T: SelfConsumable>(&mut self, literal: &T) -> Result<usize, ConsumeError>;

    /// A shorthand for the [`consume_from`](trait.Consumable.html#tymethod.consume_from).
    /// Here the `source` is `self`.
    ///
    /// Will mutate `source` to have the unconsumed part.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), manger::error::ConsumeError> {
    /// use manger::ConsumeSource;
    ///
    /// let mut source = "{42}";
    ///
    /// source.mut_consume_lit(&'{')?;
    /// assert_eq!(source, "42}");
    ///
    /// let num = source.mut_consume::<u32>()?;
    /// assert_eq!(num, 42);
    /// assert_eq!(source, "}");
    ///
    /// source.mut_consume_lit(&'}')?;
    /// assert_eq!(source, "");
    /// # Ok(())
    /// # }
    /// ```
    fn mut_consume<T: Consumable>(&mut self) -> Result<T, ConsumeError>;

    /// A shorthand for the [`consume_how_many_from`](trait.Consumable.html#tymethod.consume_from).
    /// Here the `source` is `self`.
    ///
    /// Will mutate `source` to have the unconsumed part.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), manger::error::ConsumeError> {
    /// use manger::ConsumeSource;
    ///
    /// let mut source = "{42}";
    ///
    /// source.mut_consume_lit(&'{')?;
    /// assert_eq!(source, "42}");
    ///
    /// let (num, amount) = source.mut_consume_by::<u32>()?;
    /// assert_eq!(num, 42);
    /// assert_eq!(amount, 2);
    /// assert_eq!(source, "}");
    ///
    /// source.mut_consume_lit(&'}')?;
    /// assert_eq!(source, "");
    /// # Ok(())
    /// # }
    /// ```
    fn mut_consume_by<T: Consumable>(&mut self) -> Result<(T, usize), ConsumeError>;
}

impl<'s> ConsumeSource for &'s str {
    fn consume_lit<T: SelfConsumable>(self, item: &T) -> Result<Self, ConsumeError> {
        <T>::consume_item(self, item)
    }

    fn consume<T: Consumable>(self) -> Result<(T, Self), ConsumeError> {
        <T>::consume_from(self)
    }

    fn mut_consume<T: Consumable>(&mut self) -> Result<T, ConsumeError> {
        let (item, unconsumed) = self.consume()?;
        *self = unconsumed;

        Ok(item)
    }

    fn mut_consume_lit<T: SelfConsumable>(&mut self, literal: &T) -> Result<usize, ConsumeError> {
        let length = utf8_slice::len(self);

        let unconsumed = self.consume_lit(literal)?;
        *self = unconsumed;

        Ok(length - utf8_slice::len(self))
    }

    fn mut_consume_by<T: Consumable>(&mut self) -> Result<(T, usize), ConsumeError> {
        let length = utf8_slice::len(self);
        let (item, unconsumed) = self.consume()?;
        *self = unconsumed;

        Ok((item, length - utf8_slice::len(self)))
    }
}

#[doc(inline)]
pub use iter::ConsumeIter;

pub mod chars;
pub mod common;
mod consumable;
mod either;
mod enum_macro;
mod error;
mod floats;
mod impls;
mod integers;
mod iter;
mod strs;
mod struct_macro;

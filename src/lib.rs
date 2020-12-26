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

use error::ConsumeError;

/// Consume one or more of type _T_.
/// This is equalent of the `+` operator in EBNF syntax or within RegEx.
///
///
#[derive(Debug)]
pub struct OneOrMore<T: Consumable> {
    /// The element that is guarenteed to be consumed
    head: T,
    /// Other items had are possibly parsed
    tail: Vec<T>,
}

impl<T: Consumable> OneOrMore<T> {
    /// Getter for the first item of the `OneOrMore<T>`. Because there were one or more
    /// consumed, this will always contain an item.
    pub fn head(&self) -> &T {
        &self.head
    }

    /// Getter for the rest of the element in the `OneOrMore<T>`. This is not guarenteed
    /// to contain elements.
    pub fn tail(&self) -> &Vec<T> {
        &self.tail
    }

    /// Take ownership `self` of type `OneOrMore<T>` and return a `Vec<T>` owning all
    /// the items `self` used to contain.
    ///
    /// # Examples
    ///
    /// ```
    /// use manger::{OneOrMore, Consumable};
    ///
    /// let (items, _) = <OneOrMore<char>>::consume_from("aBcdEFg")?;
    ///
    /// let uppercased: String = items
    ///     .into_vec()
    ///     .into_iter()
    ///     .filter(|character| character.is_ascii_uppercase())
    ///     .collect();
    ///
    /// assert_eq!(uppercased, "BEF");
    /// # Ok::<(), manger::error::ConsumeError>(())
    /// ```
    pub fn into_vec(self) -> Vec<T> {
        let mut vec = Vec::with_capacity(self.tail().len() + 1);

        vec.push(self.head);
        self.tail.into_iter().for_each(|item| vec.push(item));

        vec
    }

    /// Returns a vector with references to the items in the `OneOrMore<T>`.
    /// This will not take ownership of the the items in `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use manger::{OneOrMore, Consumable};
    ///
    /// let (items, _) = <OneOrMore<char>>::consume_from("aBcdEFg")?;
    ///
    /// let uppercased: String = items
    ///     .ref_vec()
    ///     .into_iter()
    ///     .filter(|character| character.is_ascii_uppercase())
    ///     .collect();
    ///
    /// assert_eq!(uppercased, "BEF");
    /// # Ok::<(), manger::error::ConsumeError>(())
    /// ```
    pub fn ref_vec(&self) -> Vec<&T> {
        let mut vec = Vec::with_capacity(self.tail().len() + 1);

        vec.push(&self.head);
        self.tail.iter().for_each(|item| vec.push(&item));

        vec
    }

    /// Returns a vector with mutable references to the items in the `OneOrMore<T>`.
    /// This will not take ownership of the the items in `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use manger::{OneOrMore, Consumable};
    ///
    /// let (mut items, _) = <OneOrMore<char>>::consume_from("aBcdEFg")?;
    ///
    /// items
    ///     .mut_vec()
    ///     .iter_mut()
    ///     .filter(|character| character.is_ascii_uppercase())
    ///     .for_each(|character| **character = character.to_ascii_lowercase());
    ///
    /// let lowercased: String = items.into_iter().collect();
    ///
    /// assert_eq!(lowercased, "abcdefg");
    /// # Ok::<(), manger::error::ConsumeError>(())
    /// ```
    pub fn mut_vec(&mut self) -> Vec<&mut T> {
        let mut vec = Vec::with_capacity(self.tail().len() + 1);

        vec.push(&mut self.head);
        self.tail.iter_mut().for_each(|item| vec.push(item));

        vec
    }
}

impl<T: Consumable> IntoIterator for OneOrMore<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_vec().into_iter()
    }
}

impl<T: Consumable> Consumable for OneOrMore<T> {
    fn consume_from(s: &str) -> Result<(Self, &str), ConsumeError> {
        let (head, unconsumed) = T::consume_from(s)?;
        let (tail, unconsumed) = <Vec<T>>::consume_from(unconsumed)?;

        Ok((OneOrMore { head, tail }, unconsumed))
    }
}
/// Consume one or more with a delimiter between elements
pub type MultipleWithDelimiter<T, D> = (Vec<(T, D)>, T);

/// Trait used to do efficient parsing.
pub trait Consumable: Sized {
    /// Attempt consume from `source` to form an item of `Self`. When consuming is
    /// succesful, it returns the item along with the unconsumed part of the source.
    /// When consuming is unsuccesful it returns the corresponding error.
    ///
    /// This is the core function to implement when implementing the
    /// [`Consumable`](#) trait.
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
    /// use manger::Consumable;
    ///
    /// let source = "42 is the answer!";
    ///
    /// let (answer, unconsumed) = u32::consume_from(source)?;
    ///
    /// assert_eq!(answer, 42);
    /// assert_eq!(unconsumed, " is the answer!");
    /// # Ok(())
    /// # }
    /// ```
    fn consume_from(source: &str) -> Result<(Self, &str), ConsumeError>;

    /// Attempt consume from `source` to form an item of `Self`. When consuming is
    /// succesful, it returns the item along with the unconsumed part of the source
    /// and the amount of consumed characters.
    /// When consuming is unsuccesful it returns the corresponding error.
    ///
    /// # Note
    ///
    /// This counts UTF-8 characters and not byte indices. This can create some
    /// confusion when slicing afterwards. One can use a crate such as
    /// [`utf8-slice`](https://crates.io/crates/utf8_slice) to compensate for this.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), manger::error::ConsumeError> {
    /// use manger::Consumable;
    ///
    /// let source = "42 is the answer!";
    ///
    /// let (answer, unconsumed, consumed_amount) = u32::consume_how_many_from(source)?;
    ///
    /// assert_eq!(answer, 42);
    /// assert_eq!(unconsumed, " is the answer!");
    /// assert_eq!(consumed_amount, 2);
    /// # Ok(())
    /// # }
    /// ```
    fn consume_how_many_from(source: &str) -> Result<(Self, &str, usize), ConsumeError> {
        let start_len = utf8_slice::len(source);
        let (item, unconsumed) = Self::consume_from(source)?;
        let end_len = utf8_slice::len(unconsumed);

        Ok((item, unconsumed, start_len - end_len))
    }
}

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
        match <T>::consume_from(self.unconsumed) {
            Ok((item, unconsumed)) => {
                self.unconsumed = unconsumed;

                Some(item)
            }
            Err(_) => None,
        }
    }
}

pub mod chars;
mod either;
pub mod error;
pub mod floats;
mod impls;
pub mod integers;
pub mod macros;
pub mod standard;
mod strs;
pub mod util;

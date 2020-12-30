#![warn(
    future_incompatible,
    rust_2018_idioms,
    missing_docs,
    missing_doc_code_examples,
    missing_debug_implementations
)]
//! # A performant, low-level, lightweight and intuitive combinatoric parser library.
//!
//! Manger allows for translation of the intuition developed for _Rust_'s primitive and standard
//! library types into your intuition for using this library. Most of the behaviour is defined with
//! the [`Consumable`] trait, which can be easily implemented using the [`consume_struct`] and
//! [`consume_enum`] macros.
//!
//! This library is suited for deterministic regular languages. It is optimally used in addition to
//! a predefined syntax. For example, if you have a predefined
//! [EBNF](https://en.wikipedia.org/wiki/Extended_Backus–Naur_form), it is really easy to
//! implement the syntax within this crate.
//!
//! # Getting Started
//!
//! To get started with implementing [`Consumable`] on your own traits, I suggest taking a look at
//! the [`consume_struct`] or [`consume_enum`] documentation. Then you can come back here and look
//! at some common patterns.
//!
//! ## Common patterns
//!
//! Parsing and thus consuming has a lot of often used patterns. Ofcourse, these are very easily
//! available here aswell.
//!
//! ### Concatenation
//!
//! Often we want to express that two patterns follow eachother in a `source` string. For example,
//! you might want to express that every `Line` is followed by a `';'`. In manger there are two
//! ways to do this.
//!
//! #### Macro's
//!
//! The first way, and the preferred way, is with the [`consume_struct`] or [`consume_enum`] macros
//! where you can present sequential consume instructions. You can see in the following example that
//! we are first consuming a `'('`, followed by a [`i32`], followed by a closing `')'`.
//!
//! ```
//! use manger::{ Consumable, consume_struct };
//!
//! struct EncasedInteger(i32);
//! consume_struct!(
//!     EncasedInteger => [
//!         > '(',
//!         value: i32,
//!         > ')';
//!         (value)
//!     ]
//! );
//! ```
//!
//! #### Tuples
//!
//! Another way to represent the same concept is with the tuple type syntax. This can be done with
//! up to 10 types. Here we are again parsing the same `(i32)` structure.
//!
//! ```
//! use manger::chars;
//!
//! type EncasedInteger = (chars::OpenParenthese, i32, chars::CloseParenthese);
//! ```
//!
//! ### Repetition
//!
//! Most of the time you want to represent some kind of repetition. There are a lot of different
//! way to represent repetition. Here there are two easy ways.
//!
//! #### Vec
//!
//! The easiest way to do repetition is with the [`Vec<T>`][std::vec::Vec]. This will consume 0 or
//! more instances of type `T`. Ofcourse, the type `T` has have has [`Consumable`] implemented.
//! Here you can see how what that looks like:
//!
//! > Since [`Vec<T>`][std::vec::Vec] will consume instances of type `T` until it finds a error, it
//! can never fail itself. You are therefore safe to unwrap the result.
//!
//! ```
//! use manger::{ Consumable, consume_struct };
//!
//! struct EncasedInteger(i32);
//! consume_struct!(
//!     EncasedInteger => [
//!         > '[',
//!         value: i32,
//!         > ']';
//!         (value)
//!     ]
//! );
//!
//! let source = "[3][-4][5]";
//!
//! let (encased_integers, _) = <Vec<EncasedInteger>>::consume_from(source)?;
//!
//! let sum = encased_integers
//!     .iter()
//!     .map(|EncasedInteger(value)| value)
//!     .sum();
//!
//! assert_eq!(sum, 4);
//! # Ok::<(), manger::ConsumeError>(())
//! ```
//!
//! #### OneOrMore
//!
//! The other easy way to do repetition is with [`OneOrMore<T>`][common::OneOrMore]. This allows for
//! consuming 1 or more instances of type `T`. And again, type `T` has to have [`Consumable`]
//! implemented. Here you can see what that looks like:
//!
//! ```
//! use manger::{ Consumable, consume_struct };
//! use manger::common::OneOrMore;
//!
//! struct EncasedInteger(i32);
//! consume_struct!(
//!     EncasedInteger => [
//!         > '[',
//!         value: i32,
//!         > ']';
//!         (value)
//!     ]
//! );
//!
//! let source = "[3][-4][5]";
//!
//! let (encased_integers, _) = <OneOrMore<EncasedInteger>>::consume_from(source)?;
//!
//! let product = encased_integers
//!     .iter()
//!     .map(|EncasedInteger(value)| value)
//!     .product();
//!
//! assert_eq!(product, -60);
//! # Ok::<(), manger::ConsumeError>(())
//! ```
//!
//! ### Optional value
//!
//! To express optional values you can use the [`Option<T>`][std::option::Option] standard rust
//! type. This will consume either 0 or 1 of type `T`.
//!
//! > Since [`Option<T>`][std::option::Option] will consume a instance of type `T` if it finds no error, it
//! can never fail itself. You are therefore safe to unwrap the result.
//!
//! ```
//! use manger::consume_struct;
//! use manger::chars;
//!
//! struct PossiblyEncasedInteger(i32);
//! consume_struct!(
//!     PossiblyEncasedInteger => [
//!         : Option<chars::OpenParenthese>,
//!         value: i32,
//!         : Option<chars::CloseParenthese>;
//!         (value)
//!     ]
//! );
//! ```
//!
//! ### Recursion
//!
//! Another common pattern seen within combinatoric parsers is recursion. Since rust types need to
//! have a predefined since, we cannot do direct type recursion and we need to do heap allocation
//! with the [`Box<T>`][std::box::Box] type from the standard library. We can make a prefixed
//! math expression parser as followed:
//!
//! ```
//! use manger::consume_enum;
//! use manger::common::{OneOrMore, Whitespace};
//!
//! enum Expression {
//!     Times(Box<Expression>, Box<Expression>),
//!     Plus(Box<Expression>, Box<Expression>),
//!     Constant(u32),
//! }
//!
//! consume_enum!(
//!     Expression {
//!         Times => [
//!             > '*',
//!             : OneOrMore<Whitespace>,
//!             left: Box<Expression>,
//!             : OneOrMore<Whitespace>,
//!             right: Box<Expression>;
//!             (left, right)
//!         ],
//!         Plus => [
//!             > '+',
//!             : OneOrMore<Whitespace>,
//!             left: Box<Expression>,
//!             : OneOrMore<Whitespace>,
//!             right: Box<Expression>;
//!             (left, right)
//!         ],
//!         Constant => [
//!             value: u32;
//!             (value)
//!         ]
//!     }
//! );
//! ```
//!
//! ### Whitespace
//!
//! For whitespace we can use the [`manger::common::Whitespace`] struct. This will consume any
//! utf-8 character that is identified as a whitespace character by the [`char::is_whitespace`]
//! function.
//!
//! ### Either
//!
//! If two possibilities are present for consuming there are two options to choose from. Both are
//! valid in certain scenarios.
//!
//! ## Macro
//!
//! Using the [`consume_enum`] you can create an struct which can be consuming in a number of
//! options and you can see which option was selected. If you need to see which of the different
//! options was selected, this should be your choice.
//!
//! ## Either<L, R>
//!
//! You can also use the [`Either<L, R>`][either::Either] type to represent the either
//! relationship. This option is preferred if we do not care about which option is selected.

#[doc(inline)]
pub use error::{ConsumeError, ConsumeErrorType};

/// Trait that defines whether a trait can be interpretted for a `source` string or not. It is the
/// trait that defines most behaviour for [manger][crate].
///
/// [`Consumable`] allows for taking a part of the start of a `source` string and turn it into a
/// instance of `Self` and the unconsumed part of the `source`.
///
/// # Implementation
///
/// Most of the implementations for this trait can be done with the [`consume_struct`] or
/// [`consume_enum`]. It is also the preferred way to implement [`Consumable`] for most types since
/// it handles error handling properly as well.
///
/// ## Custom implementations
///
/// This trait can be implemented for types by implementing the
/// [`consume_from`][Consumable::consume_from] function. The
/// [`consume_from`][Consumable::consume_from] function takes a `source` string and outputs the
/// instance of `Self` and the unconsumed part of the `source` or will return how the consuming
/// failed.
///
/// It is highly suggested that the implementation of consume_from takes into account
/// [utf-8](https://en.wikipedia.org/wiki/UTF-8), since most functions in [manger][crate] work with
/// the [utf-8](https://en.wikipedia.org/wiki/UTF-8) standard. This can be more easily done crates
/// like [utf8-slice][utf8_slice], which allows for using utf-8 character indicices in slices
/// instead of using byte indices.
///
/// # Examples
///
/// ```
/// use manger::{ Consumable, consume_struct };
///
/// let source = "[3][4][-5]";
///
/// struct EncasedInteger(i32);
/// consume_struct!(
///     EncasedInteger => [
///         > '[',
///         value: i32,
///         > ']';
///         (value)
///     ]
/// );
///
/// let product: i32 = EncasedInteger::consume_iter(source)
///     .map(|EncasedInteger(value)| value)
///     .product();
///
/// assert_eq!(product, -60);
/// ```
pub trait Consumable: Sized {
    /// Attempt consume from `source` to form an item of `Self`. When consuming is
    /// succesful, it returns the item along with the unconsumed part of the source.
    /// When consuming is unsuccesful it returns the corresponding error.
    ///
    /// This is the core function to implement when implementing the
    /// [`Consumable`](#) trait.
    ///
    /// # Implementation note
    ///
    /// It is highly recommended to take into account UTF-8 characters. This is
    /// reasonably easy with `.chars()` on `&str` or with the crate
    /// [`utf8-slice`](https://crates.io/crates/utf8_slice).
    ///
    /// # Examples
    ///
    /// ```
    /// use manger::Consumable;
    ///
    /// let source = "42 is the answer!";
    ///
    /// let (answer, unconsumed) = u32::consume_from(source)?;
    ///
    /// assert_eq!(answer, 42);
    /// assert_eq!(unconsumed, " is the answer!");
    /// # Ok::<(), manger::ConsumeError>(())
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
    /// use manger::Consumable;
    ///
    /// let source = "42 is the answer!";
    ///
    /// let (answer, unconsumed, consumed_amount) = u32::consume_how_many_from(source)?;
    ///
    /// assert_eq!(answer, 42);
    /// assert_eq!(unconsumed, " is the answer!");
    /// assert_eq!(consumed_amount, 2);
    /// # Ok::<(), manger::ConsumeError>(())
    /// ```
    fn consume_how_many_from(source: &str) -> Result<(Self, &str, usize), ConsumeError> {
        let start_len = utf8_slice::len(source);
        let (item, unconsumed) = Self::consume_from(source)?;
        let end_len = utf8_slice::len(unconsumed);

        Ok((item, unconsumed, start_len - end_len))
    }

    /// Fetch a iterator of `source` to inorderly consume items of `Self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use manger::{ Consumable, consume_struct };
    ///
    /// let source = "(3)(4)(5)";
    ///
    /// struct EncasedInteger(u32);
    /// consume_struct!(
    ///     EncasedInteger => [
    ///         > '(',
    ///         value: u32,
    ///         > ')';
    ///         (value)
    ///     ]
    /// );
    ///
    /// let product: u32 = EncasedInteger::consume_iter(source)
    ///     .map(|EncasedInteger(value)| value)
    ///     .product();
    ///
    /// assert_eq!(product, 60);
    /// ```
    fn consume_iter<'a>(source: &'a str) -> ConsumeIter<'a, Self> {
        ConsumeIter {
            phantom: std::marker::PhantomData,
            unconsumed: source,
        }
    }
}

/// Trait which allows for consuming of instances and literals from a string.
///
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
    /// # Implementation note
    ///
    /// It is highly recommended to take into account UTF-8 characters. This is
    /// reasonably easy with `.chars()` on `&str` or with the crate
    /// [`utf8-slice`](https://crates.io/crates/utf8_slice).
    ///
    /// # Examples
    ///
    /// ```
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
    /// # Ok::<(), manger::ConsumeError>(())
    /// ```
    fn consume_item<'a>(source: &'a str, item: &'_ Self) -> Result<&'a str, ConsumeError>;
}

/// Trait that exposes some functions for easier consuming syntax on `&str`.
///
/// ConsumeSource is only implemented for `&str`.
pub trait ConsumeSource: Sized {
    /// A shorthand for the [`consume_item`](trait.SelfConsumable.html#tymethod.consume_item).
    /// Here the `source` is `self` and the `item` is `literal`.
    ///
    /// # Examples
    ///
    /// ```
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
    /// # Ok::<(), manger::ConsumeError>(())
    /// ```
    fn consume_lit<T: SelfConsumable>(self, literal: &T) -> Result<Self, ConsumeError>;

    /// A shorthand for the [`consume_from`](trait.Consumable.html#tymethod.consume_from).
    /// Here the `source` is `self`. Returns how many utf-8 characters where consumed, when
    /// succesful.
    ///
    /// # Examples
    ///
    /// ```
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
    /// # Ok::<(), manger::ConsumeError>(())
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
    /// # Ok::<(), manger::ConsumeError>(())
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
    /// # Ok::<(), manger::ConsumeError>(())
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
    /// # Ok::<(), manger::ConsumeError>(())
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

/// Iterator over a `source` for a `Consumable` type `T`.
///
/// Will consume items of type 'T' in the order of the `source`.
///
/// # Examples
///
/// ```
/// use manger::{ Consumable, consume_struct };
///
/// let source = "(3)(4)(5)";
///
/// struct EncasedInteger(u32);
/// consume_struct!(
///     EncasedInteger => [
///         > '(',
///         value: u32,
///         > ')';
///         (value)
///     ]
/// );
///
/// let product: u32 = EncasedInteger::consume_iter(source)
///     .map(|EncasedInteger(value)| value)
///     .product();
///
/// assert_eq!(product, 60);
/// ```
#[derive(Debug)]
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
        let (item_option, unconsumed) = <Option<T>>::consume_from(self.unconsumed).unwrap();
        self.unconsumed = unconsumed;

        item_option
    }
}

pub mod chars;
pub mod common;
mod either;
mod enum_macro;
mod error;
mod floats;
mod impls;
mod integers;
mod strs;
mod struct_macro;

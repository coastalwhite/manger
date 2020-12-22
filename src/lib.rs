//! *Manger*
//! _A performant, low-level, lightweight and intuitive parsing library_
//!
//! ## Why use Manger?
//! Manger is really easy to use, easy to understand and really performant.
//! It is inspired by combinatoric parsers.
//! Manger has a optimatized standard library including integers, floating-point and UTF-8.
//!
//! ## Getting Starting
//! Getting started with Manger is really easy.
//! Let's start with the core concept.
//!
//! ### Core Concept
//! To parse a `struct X`, we always need a function which kinda looks like `&str -> X`.
//! Manger uses this concept as well.
//! But instead of consuming the entire `&str`, we consume a part of the string.
//! So we get a function, which looks similar to `&str -> (X, &str)`.
//! This way we can easily combine different parsers.
//! We can chain two parsers of `struct X` and `struct Y` together
//! to get a function like `&str -> ((X, Y), &str)`.
//!
//! ### The `Hello World!` of parsing.
//! We are gonna create an arithmetic parser.
//! We can use this parser to calculate stringified equations.
//!
//! Let's start!
//!
//! #### How do equations look like
//! We can create a grammar for equations,
//! which will help a lot when trying to conceptualize what we are gonna do.
//!
//! A integer looks like this:
//! ```text
//! integer := [0-9]+
//! ```
//!
//! Then, we need to create the expressions:
//! ```text
//! expression := sum | product | priority | integer
//!
//! sum := expression '+' expression
//! product := expression '*' expression
//! priority := '(' expression ')'
//! ```
//!
//! We can use this grammar line by line to create a parser in _Manger_.
//!
//! ### Actually coding
//! The core trait used by _Manger_ is `Consumable`.
//! We need to always import this trait to use the parsing capability.
//!
//! After importing `Consumable` we can parse integers and floating points alike.
//! ``` rust
//! use manger::Consumable;
//!
//! // We can parse an integer
//! // consume_from is the function to parse.
//! let (integer, unconsumed) = u32::consume_from("42 is the answer.").unwrap();
//! assert_eq!(integer, 42);
//! assert_eq!(unconsumed, " is the answer.");
//!
//! // We can also parse a floating point number
//! let (floating_point, unconsumed) = f32::consume_from("42.0 0.42").unwrap();
//! assert_eq!(floating_point, 42.0f32);
//! assert_eq!(unconsumed, " 0.42");
//! ```
//!
//! We can also use the standard library to parse characters.
//! Here we parse an amount of money using the dollar sign from the standard library.
//! ``` rust
//! use manger::Consumable;
//! use manger::chars;
//!
//! // Consume the '$' character.
//! let (_, unconsumed) = <chars::Dollar>::consume_from("$12.50, please!").unwrap();
//! assert_eq!(unconsumed, "12.50, please!");
//!
//! // Consume the '12.50' floating point number.
//! let (floating_point, unconsumed) = f32::consume_from(unconsumed).unwrap();
//! assert_eq!(floating_point, 12.5f32);
//! assert_eq!(unconsumed, ", please!");
//! ```
//!
//! Ofcourse, we also create implement parsing for our own structs.
//! Here we create a consume parser for the string 'l33t'.
//! ``` rust
//! use manger::Consumable;
//! use manger::chars;
//! use manger::chars::CharConsumeError;
//!
//! struct L33T;
//!
//! impl Consumable for L33T {
//!     type ConsumeError = CharConsumeError;
//!
//!     fn consume_from(s: &str) -> Result<(Self, &str), Self::ConsumeError> {
//!         let (_, unconsumed) = <chars::alphabet::lower::L>::consume_from(s)?;
//!         let (_, unconsumed) = <chars::nums::Three>::consume_from(unconsumed)?;
//!         let (_, unconsumed) = <chars::nums::Three>::consume_from(unconsumed)?;
//!         let (_, unconsumed) = <chars::alphabet::lower::T>::consume_from(s)?;
//!
//!         Ok((L33T, unconsumed))
//!     }
//! }
//! ```
//!
//! And now we can use it:
//! ``` rust
//! # use manger::Consumable;
//! # use manger::chars;
//! # use manger::chars::CharConsumeError;
//! #
//! # struct L33T;
//! #
//! # impl Consumable for L33T {
//! #    type ConsumeError = CharConsumeError;
//! #
//! #    fn consume_from(s: &str) -> Result<(Self, &str), Self::ConsumeError> {
//! #       let (_, unconsumed) = <chars::alphabet::lower::L>::consume_from(s)?;
//! #       let (_, unconsumed) = <chars::nums::Three>::consume_from(unconsumed)?;
//! #       let (_, unconsumed) = <chars::nums::Three>::consume_from(unconsumed)?;
//! #       let (_, unconsumed) = <chars::alphabet::lower::T>::consume_from(unconsumed)?;
//! #
//! #       Ok((L33T, unconsumed))
//! #    }
//! # }
//! #
//! let (_, unconsumed) = <L33T>::consume_from("l33t is a hackername!").unwrap();
//! assert_eq!(unconsumed, " is a hackername!");
//! ```
//!
//! #### Removing the boilerplate
//!
//! But this is a lot of boiler-plate code for just consuming a string.
//! We can do better!
//!
//! ``` rust
//! use manger::Consumable;
//! use manger::manger_str;
//!
//! manger_str!( "l33t" => L33T );
//! let (_, unconsumed) = <L33T>::consume_from("l33t is a hackername!").unwrap();
//! assert_eq!(unconsumed, " is a hackername!");
//! ```
//!
//! ## Let's build that parser
//! We can start with some definitions from our grammar:
//! ```
//! use manger::Consumable;
//! use manger::manger_either;
//! use manger::chars;
//! use either::Either;
//!
//! // We are going to use u32 for our integers.
//! type Integer = u32;
//!
//! type Expression = manger_either!( Sum, Product, Priority, Integer );
//!
//! // Then we can form our sum from concatinating different types.
//! struct Sum(Integer);
//!
//! impl Consumable for Sum {
//!     type ConsumeError = ();
//!
//!     fn consume_from(s: &str) -> Result<(Self, &str), Self::ConsumeError> {
//!     
//!     }
//! }
//! (Expression, chars::Plus, Expression);
//! type Product = (Expression, chars::Asterisk, Expression);
//! type Priority = (chars::OpenParenthese, Expression, chars::CloseParenthese);
//! ```
use ::either::{Either, Either::Left, Either::Right};

/// Parse one or more of type _T_
pub type OneOrMore<T> = (T, Vec<T>);
/// Parse one or more with a delimiter between elements
pub type MultipleWithDelimiter<T, D> = (Vec<(T, D)>, T);

/// Trait used to do efficient parsing.
pub trait Consumable: Sized {
    type ConsumeError;

    /// Consume part of string to form an item of Self.
    fn consume_from(s: &str) -> Result<(Self, &str), Self::ConsumeError>;

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

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
//! use manger::{ mangez, Consumable };
//!
//! struct EncasedInteger(i32);
//! mangez!(
//!     EncasedInteger {
//!         [ '(', value: i32, ')' ];
//!         (value)
//!     }
//! );
//! ```
//!
//! #### Tuples
//!
//! Another way to represent the same concept is with the tuple type syntax. This can be done with
//! up to 10 types. Here we are again parsing the same `(i32)` structure.
//!
//! ```
//! use manger::std::chars;
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
//! use manger::{ mangez, Consumable };
//!
//! struct EncasedInteger(i32);
//! mangez!(
//!     EncasedInteger {
//!         [ '[', value: i32, ']' ];
//!         (value)
//!     }
//! );
//!
//! let source = "[3][-4][5]";
//!
//! let (encased_integers, _) = <Vec<EncasedInteger>>::consume_from(source)?;
//!
//! let sum: i32 = encased_integers
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
//! use manger::{ mangez, Consumable };
//! use manger::std::OneOrMore;
//!
//! struct EncasedInteger(i32);
//! mangez!(
//!     EncasedInteger {
//!         [ '[', value: i32, ']' ];
//!         (value)
//!     }
//! );
//!
//! let source = "[3][-4][5]";
//!
//! let (encased_integers, _) = <OneOrMore<EncasedInteger>>::consume_from(source)?;
//!
//! let product: i32 = encased_integers
//!     .into_iter()
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
//! use manger::{ mangez, Consumable };
//! use manger::std::chars;
//!
//! # #[derive(PartialEq, Debug)]
//! struct PossiblyEncasedInteger(i32);
//! mangez!(
//!     PossiblyEncasedInteger {
//!         [
//!             : Option<chars::OpenParenthese>,
//!             value: i32,
//!             : Option<chars::CloseParenthese>
//!         ];
//!         (value)
//!     }
//! );
//! # assert_eq!(PossiblyEncasedInteger::consume_from("(42)abc").unwrap(),
//! # (PossiblyEncasedInteger(42), "abc"));
//! # assert_eq!(PossiblyEncasedInteger::consume_from("42abc").unwrap(),
//! # (PossiblyEncasedInteger(42), "abc"));
//! ```
//!
//! ### Recursion
//!
//! Another common pattern seen within combinatoric parsers is recursion. Since rust types need to
//! have a predefined since, we cannot do direct type recursion and we need to do heap allocation
//! with the [`Box<T>`][std::boxed::Box] type from the standard library. We can make a prefixed
//! math expression parser as followed:
//!
//! ```
//! use manger::{ mangez, Consumable };
//! use manger::std::{OneOrMore, Whitespace};
//!
//! enum Expression {
//!     Times(Box<Expression>, Box<Expression>),
//!     Plus(Box<Expression>, Box<Expression>),
//!     Constant(u32),
//! }
//!
//! mangez!(
//!     Expression {
//!         Times {
//!             [
//!                 '*',
//!                 : OneOrMore<Whitespace>,
//!                 left: Box<Expression>,
//!                 : OneOrMore<Whitespace>,
//!                 right: Box<Expression>
//!             ];
//!             (left, right)
//!         },
//!         Plus {
//!             [
//!                 '+',
//!                 : OneOrMore<Whitespace>,
//!                 left: Box<Expression>,
//!                 : OneOrMore<Whitespace>,
//!                 right: Box<Expression>
//!             ];
//!             (left, right)
//!         },
//!         Constant {
//!             [ value: u32 ];
//!             (value)
//!         }
//!     }
//! );
//! ```
//!
//! ### Whitespace
//!
//! For whitespace we can use the [Whitespace][crate::common::Whitespace] struct. This will consume any
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
//! You can also use the [`Either<L, R>`][::either::Either] type to represent the either
//! relationship. This option is preferred if we do not care about which option is selected.
//!
#[doc(inline)]
pub use manger_macro::mangez;
#[doc(inline)]
pub use manger_core::*;
#[doc(inline)]
pub use manger_std as std;
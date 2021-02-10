#![warn(
    future_incompatible,
    rust_2018_idioms,
    missing_docs,
    missing_doc_code_examples,
    missing_debug_implementations
)]

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
/// use manger::{ mangez, Consumable };
///
/// let source = "[3][4][-5]";
///
/// struct EncasedInteger(i32);
/// mangez!(
///     EncasedInteger {
///         [ '[', value: i32, ']' ];
///         (value)
///     }
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
    /// use manger::{ mangez, Consumable };
    ///
    /// let source = "(3)(4)(5)";
    ///
    /// struct EncasedInteger(u32);
    /// mangez!(
    ///     EncasedInteger {
    ///         [ '(', value: u32, ')' ];
    ///         (value)
    ///     }
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
/// use manger::{ mangez, Consumable };
///
/// let source = "(3)(4)(5)";
///
/// struct EncasedInteger(u32);
/// mangez!(
///     EncasedInteger {
///         [ '(', value: u32, ')' ];
///         (value)
///     }
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

mod either;
mod error;
mod impls;
mod nums;
mod strs;

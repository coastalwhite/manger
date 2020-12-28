use crate::ConsumeError;

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

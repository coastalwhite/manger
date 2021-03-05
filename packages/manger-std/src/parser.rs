use manger_core::{Consumable, ConsumeError};

/// A wrapper to have default [FromStr][std::str::FromStr] behaviour.
///
/// # Examples
/// ```
/// use manger::{ mangez, Consumable };
/// use manger::std::Parser;
/// use std::str::FromStr;
///
/// struct EncasedInteger(i32);
/// mangez!(
///     EncasedInteger {
///         [ '(', value: i32, ')' ];
///         (value)
///     }
/// );
///
/// let parser = "(-42)".parse::<Parser<EncasedInteger>>()?;
/// let EncasedInteger(num) = parser.unwrap();
///
/// assert_eq!(num, -42);
/// # Ok::<(), manger::ConsumeError>(())
/// ```
#[derive(Debug)]
pub struct Parser<T>
where
    T: Consumable + Sized,
{
    value: T,
}

impl<T> std::str::FromStr for Parser<T>
where
    T: Consumable + Sized,
{
    type Err = ConsumeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Parser {
            value: <(T, crate::End)>::consume_from(s).map(|((v, _), _)| v)?,
        })
    }
}

impl<T> Parser<T>
where
    T: Consumable + Sized,
{
    /// Get a immutable reference to the parsed value.
    pub fn get_ref(&self) -> &T {
        &self.value
    }

    /// Get a mutable reference to the parsed value.
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }

    /// Unwrap the parser to fetch the parsed value.
    pub fn unwrap(self) -> T {
        self.value
    }
}

use crate::{Consumable, ConsumeError};

/// Collection struct which stores one or more items of type `T`.
///
/// This collection is used within the [__manger__ crate][crate]
/// to express consuming one of more of an item from a string.
/// This would be equivalent to the `+` operator in
/// [EBNF syntax](https://en.wikipedia.org/wiki/Extended_Backus–Naur_form) or
/// [RegEx](https://en.wikipedia.org/wiki/Regular_expression).
///
/// # Note
///
/// While `OneOrMore` is not iterable, the
/// [`into_iter`][crate::OneOrMore::into_iter],
/// [`into_vec`][crate::OneOrMore::into_vec],
/// [`ref_vec`][crate::OneOrMore::ref_vec] and
/// [`mut_vec`][crate::OneOrMore::mut_vec]
/// can be used to iterate/create iterators over the items contained within the structs
/// and do further data manipulation.
///
/// # Examples
///
/// ```
/// use manger::{Consumable, OneOrMore, consume_struct};
///
/// let source = "(2)(3)(7)";
///
/// // EncasedInteger will be consuming strings like "(123)" and "(42)"
/// struct EncasedInteger { value: u32 };
/// consume_struct!(
///     EncasedInteger => [
///         > '(',
///         value: u32,
///         > ')';
///     ]
/// );
///
/// let (encased_integers, _) = <OneOrMore<EncasedInteger>>::consume_from(source)?;
/// let product: u32 = encased_integers
///         .into_iter()
///         .map(|encased_integer| encased_integer.value)
///         .product();
///
/// assert_eq!(product, 42);
/// # Ok::<(), manger::error::ConsumeError>(())
/// ```
#[derive(Debug)]
pub struct OneOrMore<T> {
    /// The item that is guarenteed to be consumed
    head: T,

    /// Other items that were possibly consumed as well.
    /// The items are in the order they were consumed by.
    tail: Vec<T>,
}

impl<T> OneOrMore<T> {
    /// Getter for the first item of a `OneOrMore<T>`.
    ///
    /// This will return a reference to the item that is first
    /// consumed and therefore always contains an item.
    ///
    /// # Examples
    /// ```
    /// use manger::{OneOrMore, Consumable};
    ///
    /// let (items, _) = <OneOrMore<char>>::consume_from("aBcdEFg")?;
    ///
    /// assert_eq!(*items.head(), 'a');
    /// # Ok::<(), manger::error::ConsumeError>(())
    /// ```
    pub fn head(&self) -> &T {
        &self.head
    }

    /// Getter for the non-first items of a `OneOrMore<T>`.
    ///
    /// This will return references to the items that is were consumed
    /// after the first item and will be in order of they position within the
    /// `source` string. The returned vector possibly has __NO_ items.
    ///
    /// # Examples
    /// ```
    /// use manger::{OneOrMore, Consumable};
    ///
    /// let (items, _) = <OneOrMore<char>>::consume_from("aBcdEFg")?;
    ///
    /// assert_eq!(*items.tail().iter().collect::<String>(), "BcdEFg");
    /// # Ok::<(), manger::error::ConsumeError>(())
    /// ```
    pub fn tail(&self) -> &Vec<T> {
        &self.tail
    }

    /// Take ownership `self` of type `OneOrMore<T>` and return a `Vec<T>` owning all
    /// the items `self` used to contain.
    ///
    /// Since the ownership of the items contained within `OneOrMore<T>` will be transfered
    /// into the vector. The `OneOrMore<T>` instance cannot be used anymore afterwards.
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

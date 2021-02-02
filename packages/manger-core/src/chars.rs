//! Types for consuming individual __utf-8 characters__.
//!
//! This module contains common ASCII characters,
//! latin alphabetic letters and decimals numeric digits.

use crate::error::ConsumeError;
use crate::error::ConsumeErrorType::*;
use crate::{Consumable, SelfConsumable};

// Trait implementations for `char`
// --------------------------------

impl SelfConsumable for char {
    fn consume_item<'a>(source: &'a str, item: &'_ Self) -> Result<&'a str, ConsumeError> {
        source.chars().next().map_or(
            Err(ConsumeError::new_with(InsufficientTokens { index: 0 })),
            |token| {
                if token == *item {
                    Ok(utf8_slice::from(source, 1))
                } else {
                    Err(ConsumeError::new_with(UnexpectedToken { index: 0, token }))
                }
            },
        )
    }
}

impl Consumable for char {
    fn consume_from(s: &str) -> Result<(Self, &str), ConsumeError> {
        if let Some(token) = s.chars().next() {
            Ok((token, utf8_slice::from(s, 1)))
        } else {
            Err(ConsumeError::new_with(InsufficientTokens { index: 0 }))
        }
    }
}

// --------------------------------

macro_rules! declare_ascii {
    ( $( $struct_name:ident => $char:literal ),+ ) => {
        $(
            /// Chars that represents the respective utf-8 character.
            #[derive(Debug, PartialEq)]
            pub struct $struct_name;

            impl From<$struct_name> for char {
                fn from(_: $struct_name) -> char {
                    $char
                }
            }

            impl From<$struct_name> for u8 {
                fn from(_: $struct_name) -> u8 {
                    $char as u8
                }
            }

            crate::consume_struct!(
                $struct_name => [
                    > $char;
                ]
            );
        )+
    };
}

declare_ascii![
    Space => ' ',
    Tab => '\t',
    NewLine => '\n',

    Exclamation => '!',
    DoubleQuotes => '"',
    Hash => '#',
    Dollar => '$',
    Percent => '%',
    Ampersand => '&',
    SingleQuote => '\'',
    Asterisk => '*',
    Plus => '+',
    Comma => ',',
    Hyphen => '-',
    Period => '.',
    Slash => '/',
    Colon => ':',
    Semicolon => ';',
    Question => '?',
    At => '@',
    Backslash => '\\',
    Caret => '^',
    Underscore => '_',
    Grave => '`',
    VerticalBar => '|',
    Tilde => '~',

    OpenParenthese => '(',
    CloseParenthese => ')',
    OpenBracket => '[',
    CloseBracket => ']',
    OpenBrace => '{',
    CloseBrace => '}',

    LessThan => '<',
    MoreThan => '>',
    Equals => '='
];

/// _Structs_ to consume __decimals digits__.
pub mod num {
    declare_ascii![
        Zero => '0',
        One => '1',
        Two => '2',
        Three => '3',
        Four => '4',
        Five => '5',
        Six => '6',
        Seven => '7',
        Eight => '8',
        Nine => '9'
    ];
}

/// _Enums_ to consume letters for the __Latin Alphabet__.
pub mod alpha {
    macro_rules! letter {
        ( $( $letter:ident => [ $lower_char:literal, $upper_char:literal ] ),* ) => {
            $(
                /// `Enum` for consuming the corresponding case-independent letter.
                ///
                /// Upper-case letters gets turned into `Uppercase` and lower-case letters
                /// gets turned into `Lowercase`.
                #[derive(Debug, PartialEq)]
                pub enum $letter {
                    /// Lowercase variant of corresponding letter.
                    Lowercase,

                    /// Uppercase variant of corresponding letter.
                    Uppercase,
                }

                impl From<$letter> for char {
                    fn from(letter: $letter) -> char {
                        use $letter::*;

                        match letter {
                            Lowercase => char::from( lower::$letter ),
                            Uppercase => char::from( upper::$letter ),
                        }
                    }
                }

                impl From<$letter> for u8 {
                    fn from(letter: $letter) -> u8 {
                        use $letter::*;

                        match letter {
                            Lowercase => u8::from( lower::$letter ),
                            Uppercase => u8::from( upper::$letter ),
                        }
                    }
                }

                crate::consume_enum!(
                    $letter {
                        Lowercase => [ : lower::$letter; ],
                        Uppercase => [ : upper::$letter; ]
                    }
                );
            )*

            /// _Structs_ to consume lower-case letters for the __Latin Alphabet__.
            pub mod lower {
                declare_ascii!( $( $letter => $lower_char ),* );
            }

            /// _Structs_ to consume upper-case letters for the __Latin Alphabet__.
            pub mod upper {
                declare_ascii!( $( $letter => $upper_char ),* );
            }
        };
    }

    letter![
        A => [ 'a', 'A' ],
        B => [ 'b', 'B' ],
        C => [ 'c', 'C' ],
        D => [ 'd', 'D' ],
        E => [ 'e', 'E' ],
        F => [ 'f', 'F' ],
        G => [ 'g', 'G' ],
        H => [ 'h', 'H' ],
        I => [ 'i', 'I' ],
        J => [ 'j', 'J' ],
        K => [ 'k', 'K' ],
        L => [ 'l', 'L' ],
        M => [ 'm', 'M' ],
        N => [ 'n', 'N' ],
        O => [ 'o', 'O' ],
        P => [ 'p', 'P' ],
        Q => [ 'q', 'Q' ],
        R => [ 'r', 'R' ],
        S => [ 's', 'S' ],
        T => [ 't', 'T' ],
        U => [ 'u', 'U' ],
        V => [ 'v', 'V' ],
        W => [ 'w', 'W' ],
        X => [ 'x', 'X' ],
        Y => [ 'y', 'Y' ],
        Z => [ 'z', 'Z' ]
    ];
}

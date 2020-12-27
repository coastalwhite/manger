use crate::error::ConsumeError;
use crate::error::ConsumeErrorType::*;
use crate::{Consumable, SelfConsumable};

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

macro_rules! char_impl {
    ( $typename:ident => $char:literal ) => {
        impl $crate::Consumable for $typename {
            fn consume_from(s: &str) -> Result<(Self, &str), $crate::error::ConsumeError> {
                use $crate::error::ConsumeError;
                use $crate::error::ConsumeErrorType::*;

                match s.chars().next() {
                    Some(token) => {
                        if (token == $char) {
                            Ok(($typename, utf8_slice::from(s, 1)))
                        } else {
                            Err(ConsumeError::new_with(UnexpectedToken { index: 0, token }))
                        }
                    }
                    _ => Err(ConsumeError::new_with(InsufficientTokens { index: 0 })),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! manger_char {
    ( $typename:ident => $char:literal ) => {
        struct $typename;
        char_impl! ( $typename => $char );
    };
    ( @pub $typename:ident => $char:literal ) => {
        pub struct $typename;
        char_impl! ( $typename => $char );
    };
}

macro_rules! declare_chars {
    ( $( $typename:ident => $char:literal ),* ) => {
        $( manger_char!( @pub $typename => $char ); )*
    };
}

declare_chars![
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

pub mod nums {
    declare_chars![
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

pub mod alphabet {
    use either::Either;

    macro_rules! letter_either {
        ( $( $letter:ident ),* ) => {
            $( pub type $letter = Either<lower::$letter, upper::$letter>; )*
        };
    }

    letter_either![A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z];

    pub mod upper {
        declare_chars![
            A => 'A',
            B => 'B',
            C => 'C',
            D => 'D',
            E => 'E',
            F => 'F',
            G => 'G',
            H => 'H',
            I => 'I',
            J => 'J',
            K => 'K',
            L => 'L',
            M => 'M',
            N => 'N',
            O => 'O',
            P => 'P',
            Q => 'Q',
            R => 'R',
            S => 'S',
            T => 'T',
            U => 'U',
            V => 'V',
            W => 'W',
            X => 'X',
            Y => 'Y',
            Z => 'Z'
        ];
    }

    pub mod lower {
        declare_chars![
            A => 'a',
            B => 'b',
            C => 'c',
            D => 'd',
            E => 'e',
            F => 'f',
            G => 'g',
            H => 'h',
            I => 'i',
            J => 'j',
            K => 'k',
            L => 'l',
            M => 'm',
            N => 'n',
            O => 'o',
            P => 'p',
            Q => 'q',
            R => 'r',
            S => 's',
            T => 't',
            U => 'u',
            V => 'v',
            W => 'w',
            X => 'x',
            Y => 'y',
            Z => 'z'
        ];
    }
}

use crate::consume_struct;
pub struct Whitespace;

consume_struct!(
    Whitespace => [
        : char { |token: char| token.is_whitespace() };
    ]
);

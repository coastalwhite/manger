use either::Either;

#[derive(Debug, PartialEq)]
pub enum CharConsumeError {
    EmptyString,
    InvalidToken(usize, char),
}

impl From<(CharConsumeError, CharConsumeError)> for CharConsumeError {
    fn from(err: (Self, Self)) -> Self {
        use CharConsumeError::*;

        match err {
            (EmptyString, _) => EmptyString,
            (InvalidToken(l, c), _) => InvalidToken(l, c),
        }
    }
}

impl From<Either<CharConsumeError, CharConsumeError>> for CharConsumeError {
    fn from(err: Either<Self, Self>) -> Self {
        use ::either::{Left, Right};

        match err {
            Left(e) => e,
            Right(e) => e,
        }
    }
}

macro_rules! char_impl {
    ( $typename:ident => $char:literal ) => {
        impl crate::ConsumeParsable for $typename {
            type ConsumeError = CharConsumeError;

            fn consume(s: &str) -> Result<(Self, &str), Self::ConsumeError> {
                match s.chars().next() {
                    Some(c) => {
                        if (c == $char) {
                            Ok(($typename, utf8_slice::from(s, 1)))
                        } else {
                            Err(CharConsumeError::InvalidToken(0, c))
                        }
                    }
                    _ => Err(CharConsumeError::EmptyString),
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
    use super::CharConsumeError;

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
    use super::CharConsumeError;
    use either::Either;

    macro_rules! letter_either {
        ( $( $letter:ident ),* ) => {
            $( pub type $letter = Either<lower::$letter, upper::$letter>; )*
        };
    }

    letter_either![A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z];

    pub mod upper {
        use super::CharConsumeError;

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
        use super::CharConsumeError;

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

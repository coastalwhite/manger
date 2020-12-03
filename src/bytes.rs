use either::Either;

#[derive(Debug, PartialEq)]
pub enum ByteConsumeError {
    EmptyString,
    InvalidToken(usize, u8),
}

impl From<(ByteConsumeError, ByteConsumeError)> for ByteConsumeError {
    fn from(err: (Self, Self)) -> Self {
        use ByteConsumeError::*;

        match err {
            (EmptyString, _) => EmptyString,
            (InvalidToken(l, c), _) => InvalidToken(l, c),
        }
    }
}

impl From<Either<ByteConsumeError, ByteConsumeError>> for ByteConsumeError {
    fn from(err: Either<Self, Self>) -> Self {
        use ::either::{Left, Right};

        match err {
            Left(e) => e,
            Right(e) => e,
        }
    }
}

macro_rules! byte_impl {
    ( $typename:ident => $byte:literal ) => {
        impl crate::ASCIIConsumable for $typename {
            type ASCIIConsumeError = ByteConsumeError;

            fn ascii_consume_from(s: &str) -> Result<(Self, &str), Self::ASCIIConsumeError> {
                match s.bytes().next() {
                    Some(c) => {
                        if (c == $byte) {
                            Ok(($typename, utf8_slice::from(s, 1)))
                        } else {
                            Err(ByteConsumeError::InvalidToken(0, c))
                        }
                    }
                    _ => Err(ByteConsumeError::EmptyString),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! manger_byte {
    ( $typename:ident => $byte:literal ) => {
        struct $typename;
        byte_impl!( $typename => $byte );
    };
    ( @pub $typename:ident => $byte:literal ) => {
        pub struct $typename;
        byte_impl!( $typename => $byte );
    };
}

macro_rules! declare_bytes {
    ( $( $typename:ident => $byte:literal ),* ) => {
        $( manger_byte!( @pub $typename => $byte ); )*
    };
}

declare_bytes![
    Space => b' ',
    Tab => b'\t',
    NewLine => b'\n',

    Exclamation => b'!',
    DoubleQuotes => b'"',
    Hash => b'#',
    Dollar => b'$',
    Percent => b'%',
    Ampersand => b'&',
    SingleQuote => b'\'',
    Asterisk => b'*',
    Plus => b'+',
    Comma => b',',
    Hyphen => b'-',
    Period => b'.',
    Slash => b'/',
    Colon => b':',
    Semicolon => b';',
    Question => b'?',
    At => b'@',
    Backslash => b'\\',
    Caret => b'^',
    Underscore => b'_',
    Grave => b'`',
    VerticalBar => b'|',
    Tilde => b'~',

    OpenParenthese => b'(',
    CloseParenthese => b')',
    OpenBracket => b'[',
    CloseBracket => b']',
    OpenBrace => b'{',
    CloseBrace => b'}',

    LessThan => b'<',
    MoreThan => b'>',
    Equals => b'='
];

pub mod nums {
    use super::ByteConsumeError;

    declare_bytes![
    Zero => b'0',
    One => b'1',
    Two => b'2',
    Three => b'3',
    Four => b'4',
    Five => b'5',
    Six => b'6',
    Seven => b'7',
    Eight => b'8',
    Nine => b'9'
    ];
}

pub mod alphabet {
    use super::ByteConsumeError;
    use either::Either;

    macro_rules! letter_either {
        ( $( $letter:ident ),* ) => {
            $( pub type $letter = Either<lower::$letter, upper::$letter>; )*
        };
    }

    letter_either![A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z];
    pub mod upper {
        use super::ByteConsumeError;

        declare_bytes![
            A => b'A',
            B => b'B',
            C => b'C',
            D => b'D',
            E => b'E',
            F => b'F',
            G => b'G',
            H => b'H',
            I => b'I',
            J => b'J',
            K => b'K',
            L => b'L',
            M => b'M',
            N => b'N',
            O => b'O',
            P => b'P',
            Q => b'Q',
            R => b'R',
            S => b'S',
            T => b'T',
            U => b'U',
            V => b'V',
            W => b'W',
            X => b'X',
            Y => b'Y',
            Z => b'Z'
        ];
    }

    pub mod lower {
        use super::ByteConsumeError;

        declare_bytes![
            A => b'a',
            B => b'b',
            C => b'c',
            D => b'd',
            E => b'e',
            F => b'f',
            G => b'g',
            H => b'h',
            I => b'i',
            J => b'j',
            K => b'k',
            L => b'l',
            M => b'm',
            N => b'n',
            O => b'o',
            P => b'p',
            Q => b'q',
            R => b'r',
            S => b's',
            T => b't',
            U => b'u',
            V => b'v',
            W => b'w',
            X => b'x',
            Y => b'y',
            Z => b'z'
        ];
    }
}

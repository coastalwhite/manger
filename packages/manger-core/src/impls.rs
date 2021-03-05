use crate::Consumable;
use crate::ConsumeError;

impl<T: Consumable> Consumable for Option<T> {
    fn consume_from(source: &str) -> Result<(Option<T>, &str), ConsumeError> {
        Ok(match <T>::consume_from(source) {
            Err(_) => (None, source),
            Ok((item, unconsumed)) => (Some(item), unconsumed),
        })
    }
}

impl<T: Consumable> Consumable for Box<T> {
    fn consume_from(s: &str) -> Result<(Box<T>, &str), ConsumeError> {
        <T>::consume_from(s).map(|(item, unconsumed)| (Box::new(item), unconsumed))
    }
}

impl<T: Consumable> Consumable for Vec<T> {
    fn consume_from(s: &str) -> Result<(Vec<T>, &str), ConsumeError> {
        let mut sequence = Vec::new();
        let mut last_unconsumed = s;

        while let Ok((extra_coordinate_pair, unconsumed)) = T::consume_from(last_unconsumed) {
            sequence.push(extra_coordinate_pair);
            last_unconsumed = unconsumed;
        }

        Ok((sequence, last_unconsumed))
    }
}

// Trait implementations for `char`
// --------------------------------

use crate::ConsumeErrorType::*;
use crate::SelfConsumable;

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
use crate::ConsumeSource;

macro_rules! consume_concat {
    ( $( $type_ident:ident ),+ ) => {
        impl<$( $type_ident ),+> Consumable for ($( $type_ident ),+)
        where
            $( $type_ident: Consumable ),+
        {
            fn consume_from(source: &str) -> Result<(Self, &str), ConsumeError> {
                let mut unconsumed = source;
                let mut offset = 0;

                Ok(
                    (
                        (
                            $(
                                unconsumed
                                    .mut_consume_by::<$type_ident>()
                                    .map_err( |err| { err.offset(offset) } )
                                    .map( |(item, by)| { offset += by; item } )?
                            ),+
                        ),
                        unconsumed
                    )
                )
            }
        }
    };
}

consume_concat!(A, B);
consume_concat!(A, B, C);
consume_concat!(A, B, C, D);
consume_concat!(A, B, C, D, E);
consume_concat!(A, B, C, D, E, F);
consume_concat!(A, B, C, D, E, F, G);
consume_concat!(A, B, C, D, E, F, G, H);
consume_concat!(A, B, C, D, E, F, G, H, I);
consume_concat!(A, B, C, D, E, F, G, H, I, J);

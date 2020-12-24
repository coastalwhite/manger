// use crate::chars::{self, CharConsumeError};
// use crate::standard::{Digit, Sign};
// use crate::{Consumable, OneOrMore};
// use std::num::ParseFloatError;
// use std::str::FromStr;

// #[derive(Debug, PartialEq)]
// pub enum FloatConsumeError {
//     Overflow,
//     InvalidToken(char),
//     Float(ParseFloatError),
//     EmptyString,
// }

// impl Consumable for f32 {
//     type ConsumeError = FloatConsumeError;

//     fn consume_from(s: &str) -> Result<(Self, &str), Self::ConsumeError> {
//         let (Sign(_is_positive), unconsumed) = Sign::consume_from(s).map_err(|err| err.into())?;
//         let (integer, unconsumed) =
//             <Vec<Digit>>::consume_from(unconsumed).map_err(|either_err| either_err.into())?;

//         let unconsumed = if integer.is_empty() {
//             let (_, unconsumed) =
//                 chars::Period::consume_from(unconsumed).map_err(|err| err.into())?;
//             let (_fraction, unconsumed) =
//                 <Vec<Digit>>::consume_from(unconsumed).map_err(|err| err.into())?;

//             unconsumed
//         } else {
//             if let (Some(_), unconsumed) = chars::Period::try_consume_from(unconsumed) {
//                 let (_fraction, unconsumed) =
//                     <Vec<Digit>>::consume_from(unconsumed).map_err(|err| err.into())?;

//                 unconsumed
//             } else {
//                 unconsumed
//             }
//         };

//         let (_, unconsumed) =
//             <(chars::alphabet::E, OneOrMore<Digit>)>::try_consume_from(unconsumed);

//         Ok((
//             <f32>::from_str(utf8_slice::till(
//                 s,
//                 utf8_slice::len(s) - utf8_slice::len(unconsumed),
//             ))
//             .map_err(|err| FloatConsumeError::Float(err))?,
//             unconsumed,
//         ))
//     }
// }

#[cfg(test)]
mod tests {
    use crate::Consumable;

    #[test]
    fn test_f32_parsing() {
        // assert_eq!(f32::consume_from("1.2e12").unwrap().0, 1.2e12f32);
    }
}

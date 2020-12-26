use crate::chars;
use crate::error::ConsumeError;
use crate::error::ConsumeErrorType::*;
use crate::standard::{Digit, Sign};
use crate::{consume_syntax, Consumable, OneOrMore};
use std::str::FromStr;

enum FloatNumberStruct {
    NoPeriod,
    PeriodStart,
    PeriodMiddle,
}

consume_syntax!(
    FloatNumberStruct {
        PeriodStart => [
            : chars::Period,
            : OneOrMore<Digit>;
        ],
        PeriodMiddle => [
            : OneOrMore<Digit>,
            : chars::Period,
            : OneOrMore<Digit>;
        ],
        NoPeriod => [
            : OneOrMore<Digit>;
        ]
    }
);

enum FloatStructure {
    Float,
    Infinity,
    NaN,
}

use crate::chars::alphabet;
consume_syntax!(
    FloatStructure {
        Float => [
            : Sign,
            : FloatNumberStruct,
            : Option<(alphabet::E, OneOrMore<Digit>)>;
        ],
        Infinity => [
            : Sign,
            : alphabet::I,
            : alphabet::N,
            : alphabet::F,
            : alphabet::I,
            : alphabet::N,
            : alphabet::I,
            : alphabet::T,
            : alphabet::Y;
        ],
        NaN => [
            : alphabet::N,
            : alphabet::A,
            : alphabet::N;
        ]
    }
);

impl Consumable for f32 {
    fn consume_from(source: &str) -> Result<(Self, &str), ConsumeError> {
        let (_, unconsumed) = FloatNumberStruct::consume_from(source)?;

        Ok((
            <f32>::from_str(utf8_slice::till(
                source,
                utf8_slice::len(source) - utf8_slice::len(unconsumed),
            ))
            .map_err(|_| ConsumeError::new_with(InvalidValue { index: 0 }))?,
            unconsumed,
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::Consumable;

    #[test]
    fn test_f32_parsing() {
        // assert_eq!(f32::consume_from("1.2e12").unwrap().0, 1.2e12f32);
        assert_eq!(
            <Option<(
                crate::chars::alphabet::E,
                crate::OneOrMore<crate::standard::Digit>
            )>>::consume_how_many_from("e12")
            .unwrap()
            .2,
            3
        );
    }
}

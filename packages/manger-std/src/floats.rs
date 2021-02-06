use crate::chars;
use crate::{Digit, OneOrMore, Sign};
use manger_core::error::ConsumeError;
use manger_core::error::ConsumeErrorType::*;
use manger_core::{consume_enum, Consumable};
use std::str::FromStr;

enum FloatNumberStruct {
    NoPeriod,
    PeriodStart,
    PeriodMiddle,
}

consume_enum!(
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

use crate::chars::alpha;
consume_enum!(
    FloatStructure {
        Float => [
            : Sign,
            : FloatNumberStruct,
            : Option<(alpha::E, OneOrMore<Digit>)>;
        ],
        Infinity => [
            : Sign,
            : alpha::I,
            : alpha::N,
            : alpha::F,
            : alpha::I,
            : alpha::N,
            : alpha::I,
            : alpha::T,
            : alpha::Y;
        ],
        NaN => [
            : alpha::N,
            : alpha::A,
            : alpha::N;
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

use crate::consume_struct;

/// A catch-all clause for consuming.
///
/// Most often used with Enums and with the `Either<L, R>` struct.
#[derive(Debug, PartialEq)]
pub struct CatchAll;

consume_struct!(
    CatchAll => [
        > "";
    ]
);

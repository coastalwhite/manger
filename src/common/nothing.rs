use crate::consume_struct;

#[derive(Debug, PartialEq)]
pub struct Nothing;

consume_struct!(
    Nothing => [
        > "";
    ]
);

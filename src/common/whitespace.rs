/// Struct representing a Whitespace utf-8 character.
///
/// Will consume all characters which return true on [`char::is_whitespace`][char].
#[derive(Debug, PartialEq)]
pub struct Whitespace;

crate::consume_struct!(
    Whitespace => [
        : char { |token: char| token.is_whitespace() };
    ]
);

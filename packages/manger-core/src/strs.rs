use crate::error::ConsumeError;
use crate::error::ConsumeErrorType::*;
use crate::SelfConsumable;

impl SelfConsumable for &str {
    fn consume_item<'a>(source: &'a str, item: &'_ Self) -> Result<&'a str, ConsumeError> {
        let mut unconsumed = source;

        for (index, token) in item.chars().enumerate() {
            if let Some(next_char) = unconsumed.chars().next() {
                if token != next_char {
                    return Err(ConsumeError::new_with(UnexpectedToken { index, token }));
                }
            } else {
                return Err(ConsumeError::new_with(InsufficientTokens { index }));
            }

            unconsumed = utf8_slice::from(unconsumed, 1);
        }

        Ok(unconsumed)
    }
}

#[cfg(test)]
mod tests {
    use crate::SelfConsumable;

    #[test]
    fn test_strs_self_consume() {
        assert_eq!(<&str>::consume_item("ABCDEF", &"ABC"), Ok("DEF"));
    }
}

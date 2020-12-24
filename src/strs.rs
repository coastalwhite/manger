use crate::errors::StringConsumeError;
use crate::SelfConsumable;

impl SelfConsumable for &str {
    type ConsumeError = StringConsumeError;

    fn consume_item<'a, 'b>(
        item: &'a Self,
        s: &'b str,
    ) -> Result<(&'a Self, &'b str), StringConsumeError> {
        let mut unconsumed = s;

        for (index, token) in item.chars().enumerate() {
            if let Some(next_char) = unconsumed.chars().next() {
                if token != next_char {
                    return Err(StringConsumeError::UnexpectedToken { index, token });
                }
            } else {
                return Err(StringConsumeError::InsufficientTokens);
            }

            unconsumed = utf8_slice::from(unconsumed, 1);
        }

        Ok((item, unconsumed))
    }
}

#[cfg(test)]
mod tests {
    use crate::SelfConsumable;

    #[test]
    fn test_strs_self_consume() {
        let item = "ABC";
        assert_eq!(<&str>::consume_item(&item, "ABCDEF"), Ok((&item, "DEF")));
        assert_eq!(<&str>::consume_item(&item, "ABCDEF"), Ok((&item, "DEF")));
        assert_eq!(<&str>::consume_item(&item, "ABCDEF"), Ok((&item, "DEF")));
        assert_eq!(<&str>::consume_item(&item, "ABCDEF"), Ok((&item, "DEF")));
    }
}

use crate::Consumable;
use either::{Either, Either::Left, Either::Right};

impl<T: Consumable> Consumable for Option<T> {
    type ConsumeError = T::ConsumeError;

    fn consume_from(s: &str) -> Result<(Option<T>, &str), Self::ConsumeError> {
        Ok(T::try_consume_from(s))
    }
}

impl<T: Consumable> Consumable for Box<T> {
    type ConsumeError = T::ConsumeError;

    fn consume_from(s: &str) -> Result<(Box<T>, &str), Self::ConsumeError> {
        <T>::consume_from(s).map(|(item, unconsumed)| (Box::new(item), unconsumed))
    }
}

impl<T: Consumable> Consumable for Vec<T> {
    type ConsumeError = T::ConsumeError;

    fn consume_from(s: &str) -> Result<(Vec<T>, &str), Self::ConsumeError> {
        let mut sequence = Vec::new();
        let mut last_unconsumed = s;

        while let Ok((extra_coordinate_pair, unconsumed)) = T::consume_from(last_unconsumed) {
            sequence.push(extra_coordinate_pair);
            last_unconsumed = unconsumed;
        }

        Ok((sequence, last_unconsumed))
    }
}

impl<T, J> Consumable for (T, J)
where
    T: Consumable,
    J: Consumable,
{
    type ConsumeError = Either<T::ConsumeError, J::ConsumeError>;

    fn consume_from(s: &str) -> Result<(Self, &str), Self::ConsumeError> {
        let (t_item, unconsumed) = T::consume_from(s).map_err(|err| Left(err))?;
        let (j_item, unconsumed) = J::consume_from(unconsumed).map_err(|err| Right(err))?;

        Ok(((t_item, j_item), unconsumed))
    }
}

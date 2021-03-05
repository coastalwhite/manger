use crate::error::ConsumeError;
use crate::Consumable;
use either::Either;

impl<L, R> Consumable for Either<L, R>
where
    L: Consumable,
    R: Consumable,
{
    fn consume_from(s: &str) -> Result<(Self, &str), ConsumeError> {
        let left = <L>::consume_from(s);

        match left {
            Ok((left_item, unconsumed)) => Ok((Either::Left(left_item), unconsumed)),
            Err(left_err) => {
                let right = <R>::consume_from(s);

                match right {
                    Ok((right_item, unconsumed)) => Ok((Either::Right(right_item), unconsumed)),
                    Err(right_err) => {
                        let mut errors = ConsumeError::new();
                        errors.add_causes(left_err);
                        errors.add_causes(right_err);

                        Err(errors)
                    }
                }
            }
        }
    }
}

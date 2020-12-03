use super::ConsumeParsable;
use either::Either;

impl<L, R> ConsumeParsable for Either<L, R>
where
    L: ConsumeParsable,
    R: ConsumeParsable,
{
    type ConsumeError = (L::ConsumeError, R::ConsumeError);

    fn consume(s: &str) -> Result<(Self, &str), Self::ConsumeError> {
        let left = <L>::consume(s);

        match left {
            Ok((left_item, unconsumed)) => Ok((Either::Left(left_item), unconsumed)),
            Err(left_err) => {
                let right = <R>::consume(s);

                match right {
                    Ok((right_item, unconsumed)) => Ok((Either::Right(right_item), unconsumed)),
                    Err(right_err) => Err((left_err, right_err)),
                }
            }
        }
    }
}

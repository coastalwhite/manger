pub mod floats;
pub mod integers;
pub mod strs;
pub mod tokens;

pub use floats::FloatConsumeError;
pub use integers::IntegerConsumeError;
pub use strs::StringConsumeError;
pub use tokens::TokenConsumeError;

pub trait CausableConsumeError {
    fn ref_causes(&self) -> Vec<(&usize, &char)>;
    fn mut_causes(&mut self) -> Vec<(&mut usize, &mut char)>;

    fn index_cause(&self) -> Vec<&usize> {
        self.ref_causes().iter().map(|info| info.0).collect()
    }
    fn token_cause(&self) -> Vec<&char> {
        self.ref_causes().iter().map(|info| info.1).collect()
    }

    fn move_cause(&mut self, by: usize) {
        self.mut_causes()
            .into_iter()
            .for_each(|(index, _)| *index += by);
    }
}

impl CausableConsumeError for () {
    fn ref_causes(&self) -> Vec<(&usize, &char)> {
        Vec::new()
    }
    fn mut_causes(&mut self) -> Vec<(&mut usize, &mut char)> {
        Vec::new()
    }
}

impl<L, R> CausableConsumeError for (L, R)
where
    L: CausableConsumeError,
    R: CausableConsumeError,
{
    fn ref_causes(&self) -> Vec<(&usize, &char)> {
        vec![self.0.ref_causes(), self.1.ref_causes()]
            .into_iter()
            .flatten()
            .collect()
    }

    fn mut_causes(&mut self) -> Vec<(&mut usize, &mut char)> {
        vec![self.0.mut_causes(), self.1.mut_causes()]
            .into_iter()
            .flatten()
            .collect()
    }
}

impl<L, R> CausableConsumeError for either::Either<L, R>
where
    L: CausableConsumeError,
    R: CausableConsumeError,
{
    fn ref_causes(&self) -> Vec<(&usize, &char)> {
        use either::{Left, Right};

        match self {
            Left(i) => i.ref_causes(),
            Right(i) => i.ref_causes(),
        }
    }

    fn mut_causes(&mut self) -> Vec<(&mut usize, &mut char)> {
        use either::{Left, Right};

        match self {
            Left(i) => i.mut_causes(),
            Right(i) => i.mut_causes(),
        }
    }
}

pub trait OrMergableConsumeError: Sized {
    fn merge(either: either::Either<Self, Self>) -> Self {
        use either::{Left, Right};

        match either {
            Left(i) => i,
            Right(i) => i,
        }
    }
}

impl OrMergableConsumeError for TokenConsumeError {}
impl OrMergableConsumeError for StringConsumeError {}
impl OrMergableConsumeError for IntegerConsumeError {}
impl OrMergableConsumeError for FloatConsumeError {}

pub trait OffsettedFrom<F: CausableConsumeError> {
    fn offsetted_from(from: F, offset: usize) -> Self;
}

impl<F, E> OffsettedFrom<F> for E
where
    F: CausableConsumeError,
    E: CausableConsumeError + From<F>,
{
    fn offsetted_from(from: F, offset: usize) -> Self {
        let mut err = E::from(from);
        err.move_cause(offset);

        err
    }
}

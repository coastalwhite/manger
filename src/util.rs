#[macro_export]
macro_rules! manger_concat {
    ( ) => {
        $crate::standard::Empty
    };
    ( $type:ty $(,)? ) => {
        $type
    };
    ( $left_type:ty, $( $type:ty ),+ $(,)? ) => {
        ($left_type, manger_concat!($($type),+))
    };
}

#[macro_export]
macro_rules! manger_either {
    ( ) => {
        $crate::standard::Empty
    };
    ( $type:ty $(,)? ) => {
        $type
    };
    ( $left_type:ty, $($types:ty),+ $(,)? ) => {
        Either<$left_type, manger_either!($($types),+)>
    };
}

#[macro_export]
macro_rules! manger_str {
    ( @impl $str:literal => $name:ident ) => {
        impl $crate::Consumable for $name {
            type ConsumeError = $crate::chars::CharConsumeError;

            fn consume_from(s: &str) -> Result<(Self, &str), Self::ConsumeError> {
                use $crate::chars::CharConsumeError::*;

                if s.len() < $str.len() {
                    return Err(EmptyString);
                }

                if s.starts_with($str) {
                    Ok(($name, utf8_slice::from(s, utf8_slice::len($str))))
                } else {
                    // TODO: Fix this error
                    Err(InvalidToken(s.chars().next().unwrap()))
                }
            }
        }
    };
    ( $str:literal => $name:ident ) => {
        struct $name;

        manger_str! ( @impl $str => $name );
    };
    ( @pub $str:literal => $name:ident ) => {
        pub struct $name;

        manger_str! ( @impl $str => $name );
    };
}

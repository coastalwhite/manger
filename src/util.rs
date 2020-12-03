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

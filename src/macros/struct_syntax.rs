#[macro_export]
macro_rules! consume_syntax_struct {
    ( @args $struct_name:ident, $( $prop_name:ident ),*, => ( $( $prop:ident ),* ) ) => {
        $struct_name ( $( $prop ),* )
    };
    ( @args $struct_name:ident, $( $prop_name:ident ),* ) => {
        $struct_name { $( $prop_name ),* }
    };

    (
        $struct_name:ident => [
            $(
                $( $prop_name:ident: $prop_type:ty $( { $prop_transform:expr } )? )?
                $( : $cons_type:ty $( { $cons_condition:expr } )?)?
                $( > $cons_expr:expr )?
            ),*
            ;
            $( ( $( $prop:ident ),* ) )?
        ] ) => {
        impl $crate::Consumable for $struct_name {
            type ConsumeError = ();

            fn consume_from(s: &str) -> Result<(Self, &str), Self::ConsumeError> {
                #[allow(unused_imports)]
                use $crate::ConsumeSource;

                let unconsumed = s;

                $(
                    $(
                        let ($prop_name, unconsumed) = <$prop_type>::consume_from(unconsumed).map_err(|_| ())?;

                        $(
                            let $prop_name = ($prop_transform)($prop_name);
                        )?
                    )?

                    $(
                        let (_, unconsumed) = <$cons_type>::consume_from(unconsumed)
                            .map_err(|_| ())
                        $(
                            .and_then(
                                |(item, unconsumed)| {
                                    if ($cons_condition)(item) {
                                        Ok((item, unconsumed))
                                    } else {
                                        Err(())
                                    }
                                }
                            )
                        )?
                        ?;
                    )?
                    $(
                        let (_, unconsumed) = <&str>::consume(&unconsumed, &$cons_expr).map_err(|_| ())?;
                    )?
                )+

                Ok(
                    (
                        $crate::consume_syntax_struct!(
                            @args $struct_name,
                            $( $( $prop_name, )* )?
                            $( => ( $( $prop ),* ) )?
                        ),
                        unconsumed
                    )
                )
            }
        }
    };
}

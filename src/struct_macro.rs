#[macro_export]
macro_rules! consume_struct {
    (
        $struct_name:ident => [
            $(
                $( $( $prop_name:ident )?: $cons_type:ty $( { $cons_condition:expr } )?)?
                $( > $cons_expr:expr )?
            ),*
            ;
            $( ( $( $prop:expr ),* ) )?
        ] ) => {
        impl $crate::Consumable for $struct_name {
            fn consume_from(source: &str) -> Result<(Self, &str), $crate::error::ConsumeError> {
                let mut unconsumed = source;
                let mut offset = 0;

                $(
                    $(
                        $( let $prop_name = )?
                        $crate::ConsumeSource::mut_consume_by::<$cons_type>(&mut unconsumed)
                        $(
                            .and_then(
                                |(item, by)| {
                                    if ($cons_condition)(item) {
                                        Ok((item, by))
                                    } else {
                                        Err(
                                            $crate::error::ConsumeError::new_with(
                                                $crate::error::ConsumeErrorType::InvalidValue { index: offset }
                                            )
                                        )
                                    }
                                }
                            )
                        )?
                            .map(|(prop, by)| {
                                #[allow(unused_assignments)]
                                { offset += by };

                                prop
                            })
                            .map_err( |err| err.offset(offset) )?;
                    )?

                    $(
                        $crate::ConsumeSource::mut_consume_lit(&mut unconsumed, &$cons_expr)
                            .map(|by| {
                                #[allow(unused_assignments)]
                                { offset += by };
                            })
                            .map_err( |err| err.offset(offset) )?;
                    )?
                )+

                Ok(
                    (
                        $crate::consume_struct!(
                            @internal $struct_name,
                            $( $( $( $prop_name, )* )? )?
                            $( => ( $( $prop ),* ) )?
                        ),
                        unconsumed
                    )
                )
            }
        }
    };

    ( @internal $struct_name:ident, $( $prop_name:ident, )* => ( $( $prop:expr ),* ) ) => {
        $struct_name ( $( $prop ),* )
    };
    ( @internal $struct_name:ident, $( $prop_name:ident, )* ) => {
        $struct_name { $( $prop_name, ),* }
    };
}

mod enum_syntax;
mod struct_syntax;

#[macro_export]
macro_rules! consume_syntax {
    ($enum_name:ident {
            $(
                $ident:ident => [
                    $(
                        $( $prop_name:ident: $prop_type:ty $( { $prop_transform:expr } )? )?
                        $( : $cons_type:ty $( { $cons_condition:expr } )? )?
                        $( > $cons_expr:expr )?
                    ),*
                    ;
                    $(
                        ( $( $props:ident ),* )
                    )?
                ]
            ),+
        }) => {
        $crate::consume_syntax_enum! (
            $enum_name {
                $(
                    $ident=> [
                        $(
                            $( $prop_name: $prop_type $( { $prop_transform } )? )?
                            $( : $cons_type $( { $cons_condition } )? )?
                            $( > $cons_expr )?
                        ),*
                        ;
                        $(
                            ( $( $props ),* )
                        )?
                    ]
                ),+
            }
        );
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
        ]
    ) => {
        $crate::consume_syntax_struct!(
            $struct_name => [
                $(
                    $( $prop_name: $prop_type $( { $prop_transform } )? )?
                    $( : $cons_type $( { $cons_condition } )?)?
                    $( > $cons_expr )?
                ),*
                ;
                $( ( $( $prop ),* ) )?
            ]
        );
    };
}

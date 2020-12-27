#[macro_export]
macro_rules! consume_syntax {
    ( @enumargs $enum_name:ident, $ident:ident, $( $prop_name:ident ),*, => ( $( $prop:ident ),* ) ) => {
        $enum_name::$ident ( $( $prop ),* )
    };
    ( @enumargs $enum_name:ident, $ident:ident, $( $prop_name:ident ),* ) => {
        $enum_name::$ident { $( $prop_name ),* }
    };

    (
        $enum_name:ident {
            $(
                $ident:ident => [
                    $(
                        $( $prop_name:ident: $prop_type:ty $( { $prop_transform:expr } )? )?
                        $( : $cons_type:ty $( { $cons_condition:expr } )? )?
                        $( > $cons_expr:expr )?
                    ),*
                    ;
                    $(
                        ( $( $prop:ident ),* )
                    )?
                ]
            ),+
        }
    ) => {
        impl $crate::Consumable for $enum_name {
            #[allow(unconditional_recursion)]
            fn consume_from(source: &str) -> Result<(Self, &str), $crate::error::ConsumeError> {
                let mut error = $crate::error::ConsumeError::new();

                $(
                    loop {
                        #[allow(unused_imports)]
                        use $crate::ConsumeSource;

                        let mut unconsumed = source;
                        let mut offset = 0;

                        $(
                            $(
                                #[allow(unused_assignments)]
                                let $prop_name = match unconsumed.mut_consume_by::<$prop_type>() {
                                        Err(err) => {
                                            error.add_causes(err.offset(offset));
                                            break;
                                        },
                                        Ok((prop, by)) => {
                                            offset += by;
                                            prop
                                        }
                                };

                                $(
                                    let $prop_name = ($prop_transform)($prop_name);
                                )?
                            )?

                            $(
                                if let Err(err) = unconsumed.mut_consume_by::<$cons_type>()
                                    .map( |(item, by)| { offset += by; item })
                                $(
                                    .and_then(
                                        |(item, unconsumed)| {
                                            if ($cons_condition)(item) {
                                                Ok((item, unconsumed))
                                            } else {
                                                Err($crate::error::ConsumeErrorType::InvalidValue { index: offset })
                                            }
                                        }
                                    )
                                )? {
                                    error.add_causes(err.offset(offset));
                                    break;
                                }
                            )?

                            $(
                                if let Err(err) = unconsumed.mut_consume_lit(&$cons_expr)
                                    .map( |by| offset += by )
                                {
                                    error.add_causes(err.offset(offset));
                                    break;
                                }
                            )?
                        )+

                        return Ok(
                            (
                                 consume_syntax!(
                                    @enumargs
                                    $enum_name,
                                    $ident,
                                    $(
                                        $( $prop_name, )?
                                    )*
                                    $( => ( $( $prop ),* ) )?
                                ),
                                unconsumed
                            )
                        );
                    }
                )+

                Err(error)
            }
        }
    };
    ( @structargs $struct_name:ident, $( $prop_name:ident, )* => ( $( $prop:ident ),* ) ) => {
        $struct_name ( $( $prop ),* )
    };
    ( @structargs $struct_name:ident, $( $prop_name:ident, )* ) => {
        $struct_name { $( $prop_name, ),* }
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
            fn consume_from(source: &str) -> Result<(Self, &str), $crate::error::ConsumeError> {
                #[allow(unused_imports)]
                use $crate::ConsumeSource;

                let mut unconsumed = source;
                let mut offset = 0;

                $(
                    $(
                        let $prop_name = unconsumed.mut_consume_by::<$prop_type>()
                            .map(|(prop, by)| {
                                offset += by;
                                prop
                            })
                            .map_err( |err| err.offset(offset) )?;

                        $(
                            let $prop_name = ($prop_transform)($prop_name);
                        )?
                    )?

                    $(
                        unconsumed.mut_consume_by::<$cons_type>()
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
                            .map( |(_, by)| offset += by )
                            .map_err( |err| err.offset(offset) )?;
                    )?
                    $(
                        unconsumed.mut_consume_lit(&$cons_expr)
                            .map(|by| offset += by)
                            .map_err( |err| err.offset(offset) )?;
                    )?
                )+

                Ok(
                    (
                        consume_syntax!(
                            @structargs $struct_name,
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

#[cfg(test)]
mod tests {
    mod fruits {
        use crate::Consumable;

        #[derive(Debug, PartialEq)]
        enum AppleColor {
            Green,
            Red,
            Yellow,
        }

        #[derive(Debug, PartialEq)]
        enum OrangeTaste {
            Sweet,
            Sour,
        }

        #[derive(Debug, PartialEq)]
        enum Fruit {
            Apple(AppleColor),
            Banana,
            Pear,
            Orange(OrangeTaste),
        }

        consume_syntax!(
            AppleColor {
                Green => [
                    > "green";
                ],
                Red => [
                    > "red";
                ],
                Yellow => [
                    > "yellow";
                ]
            }
        );

        consume_syntax!(
            OrangeTaste {
                Sweet => [
                    > "sweet";
                ],
                Sour => [
                    > "sour";
                ]
            }
        );

        consume_syntax!(
            Fruit {
                Apple => [
                    color: AppleColor,
                    : crate::chars::Space,
                    > "apple";
                    ( color )
                ],
                Banana => [
                    > "banana";
                ],
                Pear => [
                    > "pear";
                ],
                Orange => [
                    taste: OrangeTaste,
                    : crate::chars::Space,
                    > "orange";
                    ( taste )
                ]
            }
        );

        #[test]
        fn parse_apple_color() {
            assert_eq!(
                AppleColor::consume_from("red").unwrap(),
                (AppleColor::Red, "")
            );
            assert_eq!(
                AppleColor::consume_from("redABC").unwrap(),
                (AppleColor::Red, "ABC")
            );

            assert_eq!(
                AppleColor::consume_from("green").unwrap(),
                (AppleColor::Green, "")
            );
            assert_eq!(
                AppleColor::consume_from("greenABC").unwrap(),
                (AppleColor::Green, "ABC")
            );

            assert_eq!(
                AppleColor::consume_from("yellow").unwrap(),
                (AppleColor::Yellow, "")
            );
            assert_eq!(
                AppleColor::consume_from("yellowABC").unwrap(),
                (AppleColor::Yellow, "ABC")
            );

            assert!(AppleColor::consume_from("yelloABC").is_err());
        }

        #[test]
        fn parse_orange_taste() {
            assert_eq!(
                OrangeTaste::consume_from("sweet").unwrap(),
                (OrangeTaste::Sweet, "")
            );
            assert_eq!(
                OrangeTaste::consume_from("sweetABC").unwrap(),
                (OrangeTaste::Sweet, "ABC")
            );

            assert_eq!(
                OrangeTaste::consume_from("sour").unwrap(),
                (OrangeTaste::Sour, "")
            );
            assert_eq!(
                OrangeTaste::consume_from("sourABC").unwrap(),
                (OrangeTaste::Sour, "ABC")
            );

            assert!(OrangeTaste::consume_from("souABC").is_err());
        }

        #[test]
        fn parse_fruit() {
            assert_eq!(
                Fruit::consume_from("red apple").unwrap(),
                (Fruit::Apple(AppleColor::Red), "")
            );
            assert_eq!(
                Fruit::consume_from("yellow appleABC").unwrap(),
                (Fruit::Apple(AppleColor::Yellow), "ABC")
            );

            assert_eq!(Fruit::consume_from("banana").unwrap(), (Fruit::Banana, ""));
            assert_eq!(
                Fruit::consume_from("pearABC").unwrap(),
                (Fruit::Pear, "ABC")
            );

            assert_eq!(
                Fruit::consume_from("sweet orange").unwrap(),
                (Fruit::Orange(OrangeTaste::Sweet), "")
            );

            assert!(Fruit::consume_from("souABC").is_err());
        }
    }

    mod expressions {
        #[derive(Debug, PartialEq)]
        enum Expression {
            Times(Box<Expression>, Box<Expression>),
            Plus(Box<Expression>, Box<Expression>),
            Constant(u32),
        }

        impl Expression {
            pub fn get_value(&self) -> u32 {
                use Expression::*;

                match self {
                    Times(l, r) => l.get_value() * r.get_value(),
                    Plus(l, r) => l.get_value() + r.get_value(),
                    Constant(v) => *v,
                }
            }
        }

        use crate::chars::Whitespace;
        use crate::OneOrMore;

        consume_syntax!(
            Expression {
                Times => [
                    > '*',
                    : OneOrMore<Whitespace>,
                    left: Box<Expression>,
                    : OneOrMore<Whitespace>,
                    right: Box<Expression>;
                    (left, right)
                ],
                Plus => [
                    > '+',
                    : OneOrMore<Whitespace>,
                    left: Box<Expression>,
                    : OneOrMore<Whitespace>,
                    right: Box<Expression>;
                    (left, right)
                ],
                Constant => [ value: u32; (value) ]
            }
        );

        use crate::Consumable;

        #[test]
        fn test_constant_parsing() {
            assert_eq!(
                Expression::consume_from("123").unwrap(),
                (Expression::Constant(123), "")
            );

            assert_eq!(
                Expression::consume_from("321").unwrap(),
                (Expression::Constant(321), "")
            );
        }

        #[test]
        fn test_times_parsing() {
            assert_eq!(
                Expression::consume_from("* \n 123 321").unwrap(),
                (
                    Expression::Times(
                        Box::new(Expression::Constant(123)),
                        Box::new(Expression::Constant(321))
                    ),
                    ""
                )
            );

            assert_eq!(Expression::consume_from("* 5 3").unwrap().0.get_value(), 15);
        }

        #[test]
        fn test_plus_parsing() {
            assert_eq!(
                Expression::consume_from("+ \n 123 321").unwrap(),
                (
                    Expression::Plus(
                        Box::new(Expression::Constant(123)),
                        Box::new(Expression::Constant(321))
                    ),
                    ""
                )
            );

            assert_eq!(Expression::consume_from("+ 5 3").unwrap().0.get_value(), 8);
        }

        #[test]
        fn test_errors() {
            use crate::error::ConsumeError;
            use crate::error::ConsumeErrorType::*;

            assert_eq!(
                Expression::consume_from("*x"),
                Err(ConsumeError::new_from(vec![
                    InvalidValue { index: 1 },
                    UnexpectedToken {
                        index: 0,
                        token: '*'
                    },
                    UnexpectedToken {
                        index: 0,
                        token: '*'
                    }
                ]))
            );
        }

        #[test]
        fn test_combination_parsing() {
            assert_eq!(
                Expression::consume_from("+ *\n 123 321 456").unwrap(),
                (
                    Expression::Plus(
                        Box::new(Expression::Times(
                            Box::new(Expression::Constant(123)),
                            Box::new(Expression::Constant(321))
                        )),
                        Box::new(Expression::Constant(456))
                    ),
                    ""
                )
            );

            assert_eq!(
                Expression::consume_from("+ * + * 5 3 2 1 4")
                    .unwrap()
                    .0
                    .get_value(),
                ((((5 * 3) + 2) * 1) + 4)
            );
        }
    }
}

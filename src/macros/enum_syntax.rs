#[macro_export]
macro_rules! consume_syntax_enum {
    ( @args $enum_name:ident, $ident:ident, $( $prop_name:ident ),*, => ( $( $prop:ident ),* ) ) => {
        $enum_name::$ident ( $( $prop ),* )
    };
    ( @args $enum_name:ident, $ident:ident, $( $prop_name:ident ),* ) => {
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
            type ConsumeError = ();

            #[allow(unconditional_recursion)]
            fn consume_from(s: &str) -> Result<(Self, &str), Self::ConsumeError> {
                $(
                    loop {
                        #[allow(unused_imports)]
                        use $crate::ConsumeSource;

                        let unconsumed = s;
                        $(
                            $(
                                let ($prop_name, unconsumed) = {
                                    let result = <$prop_type>::consume_from(unconsumed);
                                    if result.is_err() {
                                        break;
                                    }

                                    result.unwrap()
                                };

                                $(
                                    let $prop_name = ($prop_transform)($prop_name);
                                )?
                            )?

                            $(
                                let (_, unconsumed) = {
                                    let result = <$cons_type>::consume_from(unconsumed)
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
                                    )?;

                                    if result.is_err() {
                                        break;
                                    }

                                    result.unwrap()
                                };
                            )?

                            $(
                                let (_, unconsumed) = {
                                    let result = <&str>::consume(&unconsumed, &$cons_expr);
                                    if result.is_err() {
                                        break;
                                    }

                                    result.unwrap()
                                };
                            )?
                        )+

                        return Ok(
                            (
                                 consume_syntax_enum!(
                                    @args
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

                Err(())
            }
        }
    };
}

#[cfg(test)]
mod tests {
    mod fruits {
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

        consume_syntax_enum!(
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

        consume_syntax_enum!(
            OrangeTaste {
                Sweet => [
                    > "sweet";
                ],
                Sour => [
                    > "sour";
                ]
            }
        );

        consume_syntax_enum!(
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
            use crate::Consumable;

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
            use crate::Consumable;

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
            use crate::Consumable;

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

        consume_syntax_enum!(
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

        #[test]
        fn test_constant_parsing() {
            use crate::Consumable;

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
            use crate::Consumable;

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
            use crate::Consumable;

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
        fn test_combination_parsing() {
            use crate::Consumable;

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

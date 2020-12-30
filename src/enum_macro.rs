/// A macro used for defining the way a `enum` should be consumed.
/// It will implement [`Consumable`][crate::Consumable] for this `enum`.
///  
/// # Examples
///
/// ```
/// use manger::consume_enum;
///
/// #[derive(PartialEq, Debug)]
/// enum HowManyFruits {
///     HasBananas(u32),
///     HasGrapes(u32),
///     NotThisOne,
///     Unknown
/// }
/// consume_enum! (
///     HowManyFruits {
///         HasBananas => [
///             // Now a list of sequential instruction
///             //
///             // Note: We have a comma after every instruction, but we end
///             // with a semicolon.
///
///             // Consuming expression looks like `> EXPRESSION`
///             > "bananas:",
///
///             // Consuming arbitrary data from a certain type looks like `: TYPE`
///             //
///             // Here we use the build in Whitespace type of consume anytype
///             // whitespace character.
///             //
///             // Note: Optionally, we can suffix a type with `{ Fn(data) -> bool }` to add
///             // an extra condition for consuming. Therefore, if we would have wrote
///             // `: char { |c| c.is_whitespace() }`, it would have had the same behaviour.
///             : manger::common::Whitespace,
///
///             // Saving data looks as such `KEY: TYPE`
///             //
///             // Note: Optionally, we can suffix a type with `{ Fn(data) -> bool }` to add
///             // an extra condition for consuming. Therefore, if we could have suffixed
///             // `{ |data| data >= 5 }`, we would require the `num_of_bananas` to be at
///             // least 5.
///             num_of_bananas: u32;
///
///             // Now we can use all our saved data to define what to do
///             // with that data.
///             //
///             // Since HasBananas takes a (u32) we have to fill such a data structure.
///             (num_of_bananas)
///         ],
///
///         // We can do the same for another variant.
///         HasGrapes => [
///             > "grape boxes: ",
///             num_of_grapes_boxes: u32,
///             > ", grapes per box: ",
///             num_of_grapes_per_box: u32;
///
///             // Here we calculate how many grapes there are in total
///             (num_of_grapes_boxes*num_of_grapes_per_box)
///         ],
///         
///         // We can also add an catch-all clause.
///         Unknown => [ > ""; ]
///
///         // We can also have a variant we are not consuming to.
///         // Here we are not consuming to the `HowManyBananas::NotThisOne` variant.
///     }
/// );
///
/// // Now we can consume HowManyFruits as normal.
/// use manger::Consumable;
///
/// let source = "bananas: 5";
/// let (how_many_fruits, _) = <HowManyFruits>::consume_from(source)?;
///
/// assert_eq!(how_many_fruits, HowManyFruits::HasBananas(5));
/// # let source = "grape boxes: 2, grapes per box: 100";
/// # let (how_many_fruits, _) = <HowManyFruits>::consume_from(source)?;
/// # assert_eq!(how_many_fruits, HowManyFruits::HasGrapes(200));
/// #
/// # let source = "a grape box 2";
/// # let (how_many_fruits, _) = <HowManyFruits>::consume_from(source)?;
/// # assert_eq!(how_many_fruits, HowManyFruits::Unknown);
/// # Ok::<(), manger::ConsumeError>(())
/// ```
///
/// # Syntax
///
/// The syntax for the macro is not very complicated. Much of the intuition on the Rust primitive
/// and _std_ types can we reapplied and only a few new concepts have to be applied.
///
/// The ENBF syntax is as follows:
/// > Please note that the syntax ignores interproduction rule
/// ```enbf
/// syntax = enum_name, "{",
///             {(variant_definition, ",")}*,
///             variant_definition,
///          "}";
///
/// variant_definition = variant_name, "=>", "[",
///                         {(instruction, ",")}*,
///                         instruction, ";",
///                         [ "(", RUST_EXPR*, ")" ], # RUST_EXPR is an arbitrary rust
///                                                   # expression it can use all the RUST_IDENT
///                                                   # defined in the previous section.
///                      "]";
///
/// instruction = expr_instruction | type_instruction;
///
/// expr_instruction = ">", RUST_EXPR;    # RUST_EXPR is an arbitrary rust expression. It should
///                                       # return a instance of a type that has the `Consumable`
///                                       # trait.
///
/// type_instruction = [ RUST_IDENT ], ":", RUST_TYPE; # RUST_IDENT is an arbitrary rust identity
///                                                    # an it will assigned to that property if no
///                                                    # tuple syntax is defined.
///                                                    # RUST_TYPE is an arbitrary rust type that
///                                                    # implements `Consumable`.
/// ```
///
/// # Note
///
/// 1. Although this macro works without importing any __manger__ traits, they will also not be
/// imported afterwards. Importing traits should still be done if methods of the trait
/// are supposed to be used afterwards.
///
/// 2. This macro assumed that we are in the same module as the `enum` mentioned
/// was defined. Some undefined behaviour might occur if this macro is called
/// outside of the module the `enum` was created.
#[macro_export]
macro_rules! consume_enum {
    (
        $enum_name:ident {
            $(
                $ident:ident => [
                    $(
                        $( $( $prop_name:ident )?: $cons_type:ty $( { $cons_condition:expr } )? )?
                        $( > $cons_expr:expr )?
                    ),*
                    ;
                    $(
                        ( $( $prop:expr ),* )
                    )?
                ]
            ),+
        }
    ) => {
        impl $crate::Consumable for $enum_name {
            fn consume_from(source: &str) -> Result<(Self, &str), $crate::ConsumeError> {
                let mut error = $crate::ConsumeError::new();

                $(
                    #[allow(unconditional_recursion)]
                    loop {
                        let mut unconsumed = source;
                        let mut offset = 0;

                        $(
                            $(
                                $( let $prop_name = )?
                                match $crate::ConsumeSource::mut_consume_by::<$cons_type>(&mut unconsumed)
                                $(
                                    .and_then(
                                        |(item, unconsumed)| {
                                            if ($cons_condition)(item) {
                                                Ok((item, unconsumed))
                                            } else {
                                                Err($crate::ConsumeErrorType::InvalidValue { index: offset })
                                            }
                                        }
                                    )
                                )?
                                {
                                        Err(err) => {
                                            error.add_causes(err.offset(offset));
                                            break;
                                        },
                                        Ok((prop, by)) => {
                                            #[allow(unused_assignments)]
                                            { offset += by };
                                            prop
                                        }
                                };
                            )?

                            $(
                                if let Err(err) = $crate::ConsumeSource::mut_consume_lit(&mut unconsumed, &$cons_expr)
                                    .map(|by| {
                                        #[allow(unused_assignments)]
                                        { offset += by };
                                    }
                                    )
                                {
                                    error.add_causes(err.offset(offset));
                                    break;
                                }
                            )?
                        )+

                        return Ok(
                            (
                                 $crate::consume_enum!(
                                    @internal
                                    $enum_name,
                                    $ident,
                                    $(
                                        $( $( $prop_name, )? )?
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

    ( @internal $enum_name:ident, $ident:ident, $( $prop_name:ident ),*, => ( $( $prop:expr ),* ) ) => {
        $enum_name::$ident ( $( $prop ),* )
    };
    ( @internal $enum_name:ident, $ident:ident, $( $prop_name:ident ),* ) => {
        $enum_name::$ident { $( $prop_name ),* }
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

        consume_enum!(
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

        consume_enum!(
            OrangeTaste {
                Sweet => [
                    > "sweet";
                ],
                Sour => [
                    > "sour";
                ]
            }
        );

        consume_enum!(
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

        use crate::common::{OneOrMore, Whitespace};

        consume_enum!(
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

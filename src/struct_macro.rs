/// A macro used for defining the way a `struct` should be consumed.
/// It will implement [`Consumable`][crate::Consumable] for this `struct`.
///  
/// # Examples
///
/// ```
/// use manger::consume_struct;
///
/// #[derive(PartialEq, Debug)]
/// struct Sum(i32);
/// consume_stuct! (
///     Sum => [
///         // Now a list of sequential instruction
///         //
///         // Note: We have a comma after every instruction, but we end
///         // with a semicolon.
///
///         // Saving data looks as such `KEY: TYPE`
///         //
///         // Note: Optionally, we can suffix a type with `{ Fn(data) -> bool }` to add
///         // an extra condition for consuming. Therefore, if we could have suffixed
///         // `{ |data| data >= 5 }`, we would require the `num_of_bananas` to be at
///         // least 5.
///         left_hand: i32;
///
///         // Consuming arbitrary data from a certain type looks like `: TYPE`
///         //
///         // Here we use the build in Whitespace type of consume anytype
///         // whitespace character.
///         //
///         // Note: Optionally, we can suffix a type with `{ Fn(data) -> bool }` to add
///         // an extra condition for consuming. Therefore, if we would have wrote
///         // `: char { |c| c.is_whitespace() }`, it would create the `Whitespace` type.
///         : Vec<manger::common::Whitespace>,
///
///         // Consuming expression looks like `> EXPRESSION`
///         > "+",
///
///         // Consuming arbitrary data from a certain type looks like `: TYPE`
///         //
///         // Here we use the build in Whitespace type of consume anytype
///         // whitespace character.
///         //
///         // Note: Optionally, we can suffix a type with `{ Fn(data) -> bool }` to add
///         // an extra condition for consuming. Therefore, if we would have wrote
///         // `: char { |c| c.is_whitespace() }`, it would create the `Whitespace` type.
///         : Vec<manger::common::Whitespace>,
///
///         right_hand: i32;
///
///         // Now we can use all our saved data to define what to do
///         // with that data.
///         //
///         // Since `Sum` takes a (i32) we have to fill such a data structure.
///         (left_hand + right_hand)
///     ],
/// );
///
/// // Now we can consume `Sum` as normal.
/// use manger::Consumable;
///
/// let source = "5 + -10";
/// let (sum, _) = Sum::consume_from(source)?;
///
/// assert_eq!(sum, Sum(-5));
/// # Ok::<(), manger::error::ConsumeError>(())
/// ```
///
/// # Syntax
///
/// The syntax for the macro is not very complicated. Much of the intuition on the Rust primitive
/// and _std_ types can we reapplied and only a few new concepts have to be applied.
///
/// The ENBF syntax is as follows:
/// > Please note that the syntax ignores interproduction rule.
/// ```enbf
/// syntax = struct_name, "=>", "[",
///             {(instruction, ",")}*,
///             instruction, ";",
///             [ "(", RUST_EXPR*, ")" ], # RUST_EXPR is an arbitrary rust expression it can use all
///                                       # the RUST_IDENT defined in the previous section.
///          "]";
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

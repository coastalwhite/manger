use crate::{Consumable, ConsumeSource};

#[macro_export]
macro_rules! consume_syntax {
    (
        $struct_name:ident => [
            $(
                $( $prop_name:ident: $prop_type:ty $( { $prop_transform:expr } )? )?
                $( > $cons_type:ty $( { $cons_condition:expr } )?)?
                $( | $cons_expr:expr )?
            ),+
        ] ) => {
        impl Consumable for $struct_name {
            type ConsumeError = ();

            fn consume_from(s: &str) -> Result<(Self, &str), Self::ConsumeError> {
                let unconsumed = s;

                $(
                    $(
                        let ($prop_name, unconsumed) = <$prop_type>::consume_from(unconsumed).map_err(|_| ())?;
                        $(
                            let $prop_name = $prop_transform;
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

                Ok(($struct_name { $( $( $prop_name, )? )+ }, unconsumed))
            }
        }
    };
}

struct XYZ {
    x: u32,
    y: u32,
    z: u32,
}

struct MoveCommand {
    x: f32,
    y: f32,
}

consume_syntax!(
    XYZ => [
        | "Hier komt een XYZ: {",
        | "x: ",
          x: u32,
        | ", y: ",
          y: u32,
        | ", z: ",
          z: u32,
        | "}"
    ]
);

#[test]
fn test_func() {
    let (xyz, unconsumed) =
        XYZ::consume_from("Hier komt een XYZ: {x: 234, y: 323, z: 3244} Hallo!").unwrap();

    assert_eq!(unconsumed, " Hallo!");

    assert_eq!(xyz.x, 234);
    assert_eq!(xyz.y, 323);
    assert_eq!(xyz.z, 3244);
}

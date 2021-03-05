use manger_core::Consumable;
use manger_macro::mangez;

#[derive(PartialEq, Debug)]
enum XYZ {
    X,
    Y,
    Z,
}

mangez! {
    XYZ {
        X {
            [
                'x'
            ]
        },
        Y {
            [
                'y'
            ]
        },
        Z {
            [
                'z'
            ]
        }
    }
}

#[test]
fn test_syntax() {
    assert_eq!(XYZ::consume_from("x").unwrap(), (XYZ::X, ""));
    assert_eq!(XYZ::consume_from("y").unwrap(), (XYZ::Y, ""));
    assert_eq!(XYZ::consume_from("z").unwrap(), (XYZ::Z, ""));
}

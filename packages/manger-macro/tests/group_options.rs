use manger_core::Consumable;
use manger_macro::mangez;

#[derive(PartialEq, Debug)]
struct EncasedInteger(i32);

mangez! {
    EncasedInteger {
        {ignore_inner_whitespace}
        [
            '(',
            value: i32,
            ')'
        ];
        (value)
    }
}

#[test]
fn test_syntax() {
    assert_eq!(EncasedInteger::consume_from("( 42 )").unwrap(), (EncasedInteger(42), ""));
    assert_eq!(EncasedInteger::consume_from("(42 )").unwrap(), (EncasedInteger(42), ""));
    assert_eq!(EncasedInteger::consume_from("(42)").unwrap(), (EncasedInteger(42), ""));
}
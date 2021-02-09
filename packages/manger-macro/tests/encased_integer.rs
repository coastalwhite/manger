use manger_core::Consumable;
use manger_macro::mangez;

#[derive(Debug, PartialEq)]
struct EncasedInteger(u32);

mangez! {
    EncasedInteger {
        [
            '[',
            value: u32,
            ']'
        ]; (value)
    }
}

#[test]
fn test_correct_consuming() {
    assert!(true);
    assert_eq!(
        EncasedInteger::consume_from("[42]").unwrap(),
        (EncasedInteger(42), "")
    );
}
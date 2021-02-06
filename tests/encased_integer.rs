use manger_macro::mangez;

struct EncasedInteger(u32);

#[test]
fn test_correct_consuming() {
    assert!(true);
    mangez! {
        (
            "abc",
        )
    };
}
use manger_core::Consumable;
use manger_macro::mangez;

#[derive(Debug, PartialEq)]
struct Trim<T: Consumable>(T);

mangez! {
    Trim<T: Consumable> {
        [
            : Vec<manger_std::Whitespace>,
            value: T,
            : Vec<manger_std::Whitespace>
        ];
        (value)
    }
}

#[test]
fn test_syntax() {
    assert_eq!(<Trim<manger_std::chars::alpha::A>>::consume_from("\t A  \n").unwrap(), (Trim(manger_std::chars::alpha::A::Uppercase), ""));
}

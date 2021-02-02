use manger_macro::sequence_item;

#[test]
fn test_proc_macro() {
    println!("Hiero");
    println!("Lit: '{}'", sequence_item!("Test Literal"));
    println!("Type: '{}'", sequence_item!(: u32));
    println!("Named Type: '{}'", sequence_item!(abc: u32));
}
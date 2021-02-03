use portion_rs::*;

#[test]
fn union_empty() {
    let x = Portion::closed(1, 2);
    let y = Portion::closed(4, 5);
    assert_eq!("()", (x | y).to_string())
}

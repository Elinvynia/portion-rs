use portion_rs::*;

#[test]
fn intersection_closed() {
    let x = Portion::closed(3, 6);
    let y = Portion::closed(4, 8);
    assert_eq!("[4, 6]", (x & y).to_string())
}

#[test]
fn intersection_open() {
    let x = Portion::open(1, 4);
    let y = Portion::open(2, 5);
    assert_eq!("(2, 4)", (x & y).to_string())
}

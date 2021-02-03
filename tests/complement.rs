use portion_rs::*;

#[test]
fn complement_open() {
    let x = Portion::open(6_u8, 10_u8);
    assert_eq!("[0, 6] | [10, 255]", (-x).to_string())
}

#[test]
fn complement_closed() {
    let x = Portion::closed(2_u8, 4_u8);
    assert_eq!("[0, 2) | (4, 255]", (-x).to_string())
}

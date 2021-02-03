use portion_rs::*;

#[test]
fn display_open() {
    let x = Portion::open(2, 4);
    assert_eq!("(2, 4)", x.to_string())
}

#[test]
fn display_closed() {
    let x = Portion::closed(3, 6);
    assert_eq!("[3, 6]", x.to_string())
}

#[test]
fn display_singleton() {
    let x = Portion::singleton(5);
    assert_eq!("[5]", x.to_string())
}

#[test]
fn display_empty() {
    let x: Interval<usize> = Portion::empty();
    assert_eq!("()", x.to_string())
}

#[test]
fn display_openclosed() {
    let x = Portion::openclosed(1, 8);
    assert_eq!("(1, 8]", x.to_string())
}

#[test]
fn display_closedopen() {
    let x = Portion::closedopen(4, 9);
    assert_eq!("[4, 9)", x.to_string())
}

#[test]
fn display_multiple() {
    let x = Portion::closed(-2_i8, 2_i8);
    assert_eq!("[-128, -2) | (2, 127]", (-x).to_string());
}

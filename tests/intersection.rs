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
    assert_eq!("(2, 4)", (x & y).to_string());
}

#[test]
fn intersection_singleton() {
    let x = Portion::singleton(3);
    let y = Portion::closed(3, 4);
    assert_eq!("[3]", (x & y).to_string());

    let x = Portion::singleton(3);
    let y = Portion::openclosed(3, 4);
    assert_eq!("()", (x & y).to_string());
}

#[test]
fn intersection_variety() {
    let x = Portion::openclosed(2, 4);
    let y = Portion::closed(2, 5);
    assert_eq!("(2, 4]", (x & y).to_string());

    let x = Portion::closedopen(3, 6);
    let y = Portion::open(2, 4);
    assert_eq!("[3, 4)", (x & y).to_string());
}

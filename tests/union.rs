use portion_rs::*;

#[test]
fn union_empty() {
    let x = Portion::closed(1, 2);
    let y = Portion::closed(4, 5);
    assert_eq!("()", (x | y).to_string())
}

#[test]
fn union_open() {
    let x = Portion::open(2, 4);
    let y = Portion::open(4, 6);
    assert_eq!("(2, 6)", (x | y).to_string())
}

#[test]
fn union_closed() {
    let x = Portion::closed(-2, 4);
    let y = Portion::closed(3, 10);
    assert_eq!("[-2, 10]", (x | y).to_string())
}

#[test]
fn union_open_closed() {
    let x = Portion::open(2, 6);
    let y = Portion::closed(3, 8);
    assert_eq!("(2, 8]", (x | y).to_string());
}

#[test]
fn union_variety() {
    let x = Portion::openclosed(2, 4);
    let y = Portion::closedopen(0, 3);
    assert_eq!("[0, 4]", (x | y).to_string())
}

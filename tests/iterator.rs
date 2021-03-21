use portion_rs::*;

#[test]
fn singleton() {
    let x: Vec<u8> = Portion::singleton(2).into_iter().collect();
    assert_eq!(x, vec![2]);
}

#[test]
fn closed() {
    let x: Vec<u8> = Portion::closed(2, 5).into_iter().collect();
    assert_eq!(x, vec![2, 3, 4, 5]);
}

#[test]
fn open() {
    let x: Vec<u8> = Portion::open(2, 5).into_iter().collect();
    assert_eq!(x, vec![3, 4]);
}

#[test]
fn openclosed() {
    let x: Vec<u8> = Portion::openclosed(2, 5).into_iter().collect();
    assert_eq!(x, vec![3, 4, 5]);
}

#[test]
fn closedopen() {
    let x: Vec<u8> = Portion::closedopen(2, 5).into_iter().collect();
    assert_eq!(x, vec![2, 3, 4]);
}

#[test]
fn empty() {
    let x: Vec<u8> = Portion::empty().into_iter().collect();
    assert_eq!(x, vec![]);
}

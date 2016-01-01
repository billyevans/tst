#[macro_use] extern crate tst;

use self::tst::TSTSet;

#[test]
fn create_root() {
    let s = TSTSet::new();
    assert_eq!(0, s.len());
}

#[test]
fn insert() {
    let mut s = TSTSet::new();

    assert_eq!(true, s.insert("abc"));
    assert_eq!(1, s.len());
}

#[test]
fn insert_same() {
    let mut s = TSTSet::new();

    assert_eq!(true, s.insert("abc"));
    assert_eq!(false, s.insert("abc"));
    assert_eq!(1, s.len());
}

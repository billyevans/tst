extern crate tst;

#[cfg(test)]
use tst::tst::*;
#[test]
fn create_root() {
    let m = TST::<i32>::new();
    assert_eq!(0, m.len());
}

#[test]
fn insert() {
    let mut m = TST::<i32>::new();

    m.insert("abc", &13);
    assert_eq!(1, m.len());
}

#[test]
fn get() {
    let mut m = TST::<i32>::new();

    m.insert("abc", &13);
    assert_eq!(Some(&13), m.get("abc"));
}

#[test]
fn get_none() {
    let mut m = TST::<i32>::new();

    m.insert("abc", &13);
    assert_eq!(None, m.get("abcd"));
    assert_eq!(None, m.get(""));
}

#[test]
fn insert_few() {
    let mut m = TST::<i32>::new();

    m.insert("abcde", &13);
    m.insert("abcdf", &14);
    m.insert("abcdg", &15);
    assert_eq!(3, m.len());

    assert_eq!(Some(&13), m.get("abcde"));
    assert_eq!(Some(&14), m.get("abcdf"));
    assert_eq!(Some(&15), m.get("abcdg"));
    assert_eq!(None, m.get("abcdh"));
}

#[test]
fn replace() {
    let mut m = TST::<i32>::new();

    m.insert("abcde", &13);
    m.insert("abcde", &1);
    assert_eq!(1, m.len());

    assert_eq!(Some(&1), m.get("abcde"));
}

#[test]
fn contains() {
    let mut m = TST::<i32>::new();

    m.insert("xxxe", &13);
    assert!(!m.contains_key("abcde"));
    assert!(!m.contains_key("xxx"));
    assert!(m.contains_key("xxxe"));
}

#[test]
fn is_empty() {
    let mut m = TST::<u32>::new();

    assert_eq!(0, m.len());
    assert!(m.is_empty());

    m.insert("qwer", &10000);
    assert!(!m.is_empty());
    // TODO: add clear() and check again
}


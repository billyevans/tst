#[macro_use] extern crate tst;

use self::tst::TSTSet;
use std::iter::FromIterator;

fn prepare_data() -> TSTSet {
    tstset!(
        "BY",
        "BYGONE",
        "BYE",
        "BYLAW",
        "BYLINE",
        "BYPASS",
        "BYPATH",
        "BYPRODUCT",
        "BYROAD",
        "BYSTANDER",
        "BYTE",
        "BYWAY",
        "BYWORD",
    )
}

#[test]
fn create_root() {
    let s = TSTSet::new();
    assert_eq!(0, s.len());
}

#[test]
fn len() {
    let mut s = TSTSet::new();

    s.insert("abc");
    s.insert("abd");
    s.insert("xxx_xx");
    s.insert("wow!");
    s.insert("deadbeef");

    assert_eq!(5, s.len());

    s.remove("abc");
    s.remove("abd");
    assert_eq!(3, s.len());
}

#[test]
fn is_empty_empty() {
    let s = TSTSet::new();

    assert_eq!(true, s.is_empty());
}

#[test]
fn is_empty_false() {
    let mut s = TSTSet::new();
    s.insert("deadbeef");

    assert_eq!(false, s.is_empty());
}

#[test]
fn is_empty_true() {
    let mut s = TSTSet::new();
    s.insert("deadbeef");
    s.clear();

    assert_eq!(true, s.is_empty());
}

#[test]
fn contains_empty() {
    let s = TSTSet::new();

    assert_eq!(false, s.contains("abc"));
}

#[test]
fn contains() {
    let mut s = TSTSet::new();

    s.insert("abc");

    assert_eq!(true, s.contains("abc"));
    assert_eq!(false, s.contains("ab"));
    assert_eq!(false, s.contains("abd"));
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

#[test]
fn remove_empty() {
    let mut s = TSTSet::new();

    assert_eq!(false, s.remove("abc"));
}

#[test]
fn remove() {
    let mut s = TSTSet::new();

    s.insert("abc");
    assert_eq!(true, s.remove("abc"));
    assert_eq!(false, s.remove("abc"));
    assert_eq!(true, s.is_empty());
}

#[test]
fn iter() {
    let set = tstset!("abc", "xxx", "qwer", "asdasd");
    let expected: Vec<String> = vec![
        "abc".to_string(),
        "asdasd".to_string(),
        "qwer".to_string(),
        "xxx".to_string(),
    ];

    assert_eq!(expected, set.iter().collect::<Vec<String>>());
}

#[test]
fn into_iter() {
    let set = tstset!(
        "b",
        "a",
        "c",
        "aa",
        "zzzzxxx"
    );
    let vec = set.into_iter().collect::<Vec<String>>();
    let orig = vec![
        "a".to_string(),
        "aa".to_string(),
        "b".to_string(),
        "c".to_string(),
        "zzzzxxx".to_string(),
    ];
    assert_eq!(orig, vec);
}

#[test]
fn from_iterator_empty() {
    let vec = vec![];
    let set = TSTSet::from_iter(vec);

    assert_eq!(true, set.is_empty());
}

#[test]
fn from_iterator() {
    let vec = vec![
        "b",
        "a",
        "c",
        "a",
        "aa",
    ];

    let set = TSTSet::from_iter(vec);
    let expected = tstset!(
        "b",
        "c",
        "a",
        "aa",
    );
    assert_eq!(expected, set);
}

#[test]
fn extend() {
    let mut set = tstset!("a");
    let vec = vec![
        "b",
        "a",
        "c",
        "a",
        "aa",
    ];
    set.extend(vec);
    let expected = tstset!(
        "a",
        "b",
        "c",
        "aa",
    );
    assert_eq!(expected, set);
}

#[test]
fn wild_card_iterator_simple() {
    let s = tstset!("x", "y", "z");

    assert_eq!("xyz", s.wildcard_iter(".").collect::<String>());
}

#[test]
fn wild_card_iterator() {
    let s = prepare_data();

    assert_eq!("BYPASSBYPATH",  s.wildcard_iter("BYPA..").collect::<String>());
}

#[test]
fn longest_prefix_empty() {
    let set = tstset!("abc");

    assert_eq!("", set.longest_prefix("qwer"));
    assert_eq!("", set.longest_prefix(""));
}

#[test]
fn longest_prefix() {
    let set = tstset!(
        "abc",
        "abcd",
        "abce",
        "abca",
        "zxd",
        "add",
        "abcdef",
    );

    assert_eq!("abcd", set.longest_prefix("abcde"));
    assert_eq!("abcdef", set.longest_prefix("abcdef"));
}

#[test]
fn prefix_iterator() {
    let set = tstset!(
        "first",
        "second",
        "firstthird",
        "firstsecond",
    );

    assert_eq!("firstfirstsecondfirstthird",  set.prefix_iter("fir").collect::<String>());
}

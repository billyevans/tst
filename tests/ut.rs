extern crate tst;

use self::tst::tst::TST;
use self::tst::tst::Entry::*;

#[test]
fn create_root() {
    let m = TST::<i32>::new();
    assert_eq!(0, m.len());
}

#[test]
fn insert() {
    let mut m = TST::<i32>::new();

    assert_eq!(None, m.insert("abc", 13));
    assert_eq!(1, m.len());
}

#[test]
fn insert_2times_without_replace() {
    let mut m = TST::new();
    m.insert("abc", 37);
    assert_eq!(Some(37), m.insert("abc", 666));
    assert_eq!(Some(&666), m.get("abc"));
}

#[test]
fn get() {
    let mut m = TST::new();

    m.insert("abc", 13);
    assert_eq!(Some(&13), m.get("abc"));
}

#[test]
fn get_none() {
    let mut m = TST::new();

    m.insert("abc", 13);
    assert_eq!(None, m.get("abcd"));
    assert_eq!(None, m.get(""));
}

#[test]
fn get_mut() {
    let mut m = TST::new();
    m.insert("abc", 1);
    match m.get_mut("abc") {
        Some(x) => *x = 13,
        None => panic!(),
    }
    assert_eq!(Some(&13), m.get("abc"));
}

#[test]
fn entry_occupied() {
    let mut m = TST::new();

    m.insert("abcde", 13);
    m.insert("abcdf", 14);
    match m.entry("abcdf") {
        Vacant(_) => unreachable!(),
        Occupied(mut entry) => {
            assert_eq!(&14, entry.get());
            assert_eq!(14, entry.insert(100));
        }
    }
    assert_eq!(Some(&100), m.get("abcdf"));
}

#[test]
fn entry_occupied_remove() {
    let mut m = TST::new();

    m.insert("abcde", 13);
    m.insert("abcdf", 14);
    match m.entry("abcdf") {
        Vacant(_) => unreachable!(),
        Occupied(entry) => {
            assert_eq!(14, entry.remove());
        }
    }
    assert_eq!(None, m.get("abcdf"));
    assert_eq!(1, m.len());
}

#[test]
fn entry_occupied_update() {
    let mut m = TST::new();

    m.insert("abcde", 13);
    m.insert("abcdf", 14);
    match m.entry("abcdf") {
        Vacant(_) => unreachable!(),
        Occupied(mut entry) => {
            {
                let v = entry.get_mut();
                assert_eq!(14, *v);
                *v += 100;
            }
            {
                let v = entry.get_mut();
                assert_eq!(114, *v);
                *v += 100;
            }
        }
    }
    assert_eq!(Some(&214), m.get("abcdf"));
    assert_eq!(2, m.len());
}

#[test]
fn entry_vacant() {
    let mut m = TST::new();

    m.insert("abcde", 13);
    m.insert("abcdf", 14);
    match m.entry("abcdg") {
        Vacant(entry) => {
            assert_eq!(100, *entry.insert(100));
        },
        Occupied(_) => unreachable!(),
    }
    assert_eq!(Some(&100), m.get("abcdg"));
}

#[test]
fn insert_few() {
    let mut m = TST::new();

    m.insert("abcde", 13);
    m.insert("abcdf", 14);
    m.insert("abcdg", 15);
    assert_eq!(3, m.len());

    assert_eq!(Some(&13), m.get("abcde"));
    assert_eq!(Some(&14), m.get("abcdf"));
    assert_eq!(Some(&15), m.get("abcdg"));
    assert_eq!(None, m.get("abcdh"));
}

#[test]
fn replace() {
    let mut m = TST::<i32>::new();

    m.insert("abcde", 13);
    m.insert("abcde", 1);
    assert_eq!(1, m.len());

    assert_eq!(Some(&1), m.get("abcde"));
}

#[test]
fn contains() {
    let mut m = TST::<i32>::new();

    m.insert("xxxe", 13);
    assert!(!m.contains_key("abcde"));
    assert!(!m.contains_key("xxx"));
    assert!(m.contains_key("xxxe"));
}

#[test]
fn is_empty() {
    let mut m = TST::<u32>::new();

    assert_eq!(0, m.len());
    assert!(m.is_empty());

    m.insert("qwer", 10000);
    assert!(!m.is_empty());

    m.clear();
    assert!(m.is_empty());
}

#[test]
fn clear() {
    let mut m = TST::new();
    m.clear();
    assert_eq!(None, m.insert("abc", 11));
    assert_eq!(None, m.insert("abcd", -3));
    assert_eq!(None, m.insert("a", 2));
    m.clear();
    assert_eq!(None, m.insert("abc", 11));
    assert_eq!(None, m.insert("abcd", -3));
    assert_eq!(None, m.insert("a", 2));
}

#[test]
fn remove_from_empty() {
    let mut m = TST::<u32>::new();
    assert_eq!(None, m.remove("xxx"));
    assert_eq!(None, m.remove(""));
}

#[test]
fn remove() {
    let mut m = TST::new();
    m.insert("abc", 1);

    assert_eq!(None, m.remove(""));
    assert_eq!(None, m.remove("a"));
    assert_eq!(None, m.remove("ab"));

    assert_eq!(Some(1), m.remove("abc"));

    assert_eq!(None, m.remove("abc"));
    assert_eq!(None, m.get("abc"));
}

#[test]
fn longest_prefix_empty() {
    let mut m = TST::new();
    m.insert("abc", 1);

    assert_eq!("", m.longest_prefix("qwer"));
    assert_eq!("", m.longest_prefix(""));
}

#[test]
fn longest_prefix() {
    let mut m = TST::new();
    m.insert("abc", 1);
    m.insert("abcd", 1);
    m.insert("abce", 1);
    m.insert("abca", 1);
    m.insert("zxd", 1);
    m.insert("add", 1);
    m.insert("abcdef", 1);

    assert_eq!("abcd", m.longest_prefix("abcde"));
    assert_eq!("abcdef", m.longest_prefix("abcdef"));
}

#[test]
fn access_by_index() {
    let mut m = TST::new();

    m.insert("abc", 2);
    m.insert("abd", 1);
    m.insert("abdd", 4);

    assert_eq!(2, m["abc"]);
    assert_eq!(1, m["abd"]);
    assert_eq!(4, m["abdd"]);
}

#[test]
fn access_by_index_mut() {
    let mut m = TST::new();

    m.insert("abc", 2);
    m.insert("abd", 1);
    m.insert("abdd", 4);
    {
        let v = &mut m["abc"];
        *v += 1;
    }

    assert_eq!(3, m["abc"]);
    assert_eq!(1, m["abd"]);
    assert_eq!(4, m["abdd"]);
}

#[test]
#[should_panic]
fn access_by_wrong_index() {
    let mut m = TST::new();

    m.insert("abc", 2);
    m.insert("abd", 1);
    m.insert("abdd", 4);

    assert_eq!(3, m["a"]);
}

#[test]
fn format_empty() {
    let m = TST::<u64>::new();

    assert_eq!("{}", format!("{:?}", m));
}

#[test]
fn format() {
    let mut m = TST::<i64>::new();

    m.insert("abc", 2);
    m.insert("abd", 1);
    m.insert("abdd", 4);
    m.insert("abcdefghjkik", -169874);

    let m_str = format!("{:?}", m);
    assert_eq!(
        "{\"abc\": 2,\"abcdefghjkik\": -169874,\"abd\": 1,\"abdd\": 4,}", 
        m_str
    );
}

#[test]
fn iterator() {
    let mut m = TST::new();

    m.insert("b", 2);
    m.insert("a", 1);
    m.insert("c", 4);
    m.insert("aa", 13);

    let mut m_str = String::new();
    for x in m.iter() {
        m_str.push_str(&format!("{:?}", x));
        //println!();
    }
    assert_eq!("(\"a\", 1)(\"aa\", 13)(\"b\", 2)(\"c\", 4)", m_str);
}

#[test]
fn iterator_mut() {
    let mut m = TST::new();

    m.insert("b", 2);
    m.insert("a", 1);
    m.insert("c", 4);
    m.insert("aa", 13);

    for (_, v) in m.iter_mut() {
        *v *= 3;
    }
    assert_eq!(Some(&6), m.get("b"));
    assert_eq!(Some(&3), m.get("a"));
    assert_eq!(Some(&12), m.get("c"));
    assert_eq!(Some(&39), m.get("aa"));
}

#[test]
fn prefix_iterator_empty() {
    let mut m = TST::new();

    m.insert("bbc", 2);
    m.insert("abc", 1);
    m.insert("dbc", 4);

    let mut m_str = String::new();
    for x in m.prefix_iter("abd") {
        m_str.push_str(&format!("{:?}", x));
    }
    assert_eq!("", m_str);
}

#[test]
fn prefix_iterator() {
    let mut m = TST::new();

    m.insert("first", 1);
    m.insert("second", 2);
    m.insert("firstthird", 3);
    m.insert("firstsecond", 12);

    let mut m_str = String::new();

    for x in m.prefix_iter("fir") {
        m_str.push_str(&format!("{:?}", x));
    }
    assert_eq!("(\"first\", 1)(\"firstsecond\", 12)(\"firstthird\", 3)", m_str);
}

#[test]
fn prefix_iterator_only_one() {
    let mut m = TST::new();

    m.insert("BY", 1);
    m.insert("BYE", 2);
    m.insert("BYGONE", 3);
    m.insert("BYLAW", 4);
    m.insert("BYLINE", 5);
    m.insert("BYPASS", 6);
    m.insert("BYPATH", 7);
    m.insert("BYPRODUCT", 8);
    m.insert("BYROAD", 9);
    m.insert("BYSTANDER", 10);
    m.insert("BYTE", 11);
    m.insert("BYWAY", 12);
    m.insert("BYWORD", 13);

    let mut m_str = String::new();

    for x in m.prefix_iter("BYE") {
        m_str.push_str(&format!("{:?}", x));
    }
    assert_eq!("(\"BYE\", 2)", m_str);
}

#[test]
fn prefix_iterator_mut() {
    let mut m = TST::new();

    m.insert("first", 1);
    m.insert("second", 2);
    m.insert("firstthird", 3);
    m.insert("firstsecond", 12);

    for x in m.prefix_iter_mut("fir") {
        *x.1 -= 13;
    }
    assert_eq!(Some(&-12), m.get("first"));
    assert_eq!(Some(&2), m.get("second"));
    assert_eq!(Some(&-10), m.get("firstthird"));
    assert_eq!(Some(&-1), m.get("firstsecond"));
}

#[test]
fn keys_iterator() {
    let mut m = TST::new();
    m.insert("abc", 1);
    m.insert("bcd", 2);
    m.insert("c", 3);
    m.insert("abcd", 1);

    let mut m_str = String::new();

    for k in m.keys() {
        m_str.push_str(&format!("{:?}", k));
    }
    assert_eq!("\"abc\"\"abcd\"\"bcd\"\"c\"", m_str);
}

#[test]
fn values_iterator() {
    let mut m = TST::new();
    m.insert("abc", 1);
    m.insert("bcd", 2);
    m.insert("c", 3);
    m.insert("abcd", 13);
    m.insert("xxx", 130);

    let mut m_str = String::new();

    for v in m.values() {
        m_str.push_str(&format!("{:?} ", v));
    }
    assert_eq!("1 13 2 3 130 ", m_str);
}

#[test]
#[should_panic]
fn  overflow_stack() {
    let mut m = TST::<i32>::new();
    let mut key = String::new();

    while key.len() < 1000000 {
        key.push_str("qwertyuiopasdfghjkl;");
    }
    m.insert(&key, 1);
    assert_eq!(1, m.len());
}


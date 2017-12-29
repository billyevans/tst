#![cfg(test)]
#![feature(test)]
extern crate test;

extern crate tst;
use self::tst::TSTMap;
use self::test::Bencher;
use std::str;

fn prepare() -> TSTMap<i32> {
    let mut m = TSTMap::<i32>::new();
    let mut key: &mut [u8] = &mut [b'b'; 10];
    m.insert(str::from_utf8(key).unwrap(), 13);

    fn fill(m: &mut TSTMap<i32>, key: &mut [u8], i: usize) {
        if i >= key.len() { return; }
        for ch in &[b'b',b'a',b'c'] {
            key[i] = *ch;
            m.insert(str::from_utf8(key).unwrap(), 13);
            fill(m, key, i+1);
        }
    }
    fill(&mut m, &mut key, 0);
    m
}

#[bench]
fn insert_same(b: &mut Bencher) {
    let mut m = prepare();

    let mut k = 100;
    b.iter(|| {
        m.insert("abcabcabca", k);
        k += 1;
    });
}

#[bench]
fn get_same(b: &mut Bencher) {
    let m = prepare();

    b.iter(|| {
        m.get("abcabcabca");
    });
}

#[bench]
fn remove_same(b: &mut Bencher) {
    let mut m = prepare();

    b.iter(|| {
        m.remove("abcabcabca");
    });
}

#[bench]
fn get_none(b: &mut Bencher) {
    let m = prepare();
    b.iter(|| {
        m.get("abcabcabcad");
    });
}

#[bench]
fn iterate(b: &mut Bencher) {
    let m = prepare();
    b.iter(|| {
        for x in m.iter() {
            test::black_box(x);
        }
    });
}

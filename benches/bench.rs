use criterion::{criterion_group, criterion_main, Criterion};
use tst::TSTMap;
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

fn insert_same(c: &mut Criterion) {
    c.bench_function("insert_same", |b| {
        b.iter_with_setup(
            || {
                let m = prepare();
                let k = 100;
                (m, k)
            },
            |(mut m, mut k)| {
                m.insert("abcabcabca", k);
                k += 1;
                std::hint::black_box((m, k))
            }
        );
    });
}

fn get_same(c: &mut Criterion) {
    c.bench_function("get_same", |b| {
        b.iter_with_setup(
            || prepare(),
            |m| {
                std::hint::black_box(m.get("abcabcabca"));
            }
        );
    });
}

fn remove_same(c: &mut Criterion) {
    c.bench_function("remove_same", |b| {
        b.iter_with_setup(
            || prepare(),
            |mut m| {
                std::hint::black_box(m.remove("abcabcabca"));
            }
        );
    });
}

fn get_none(c: &mut Criterion) {
    c.bench_function("get_none", |b| {
        b.iter_with_setup(
            || prepare(),
            |m| {
                std::hint::black_box(m.get("abcabcabcad"));
            }
        );
    });
}

fn iterate(c: &mut Criterion) {
    c.bench_function("iterate", |b| {
        b.iter_with_setup(
            || prepare(),
            |m| {
                for x in m.iter() {
                    std::hint::black_box(x);
                }
            }
        );
    });
}

criterion_group!(
    benches,
    insert_same,
    get_same,
    remove_same,
    get_none,
    iterate
);

criterion_main!(benches);

extern crate tst;

use std::env;
use tst::TSTMap;

fn main() {
    let mut m = TSTMap::<Box<(Fn(i32) -> i32)>>::new();

    m.insert("add", Box::new(move |acc: i32| acc + 1));
    m.insert("sub", Box::new(move |acc: i32| acc - 1));
    m.insert("mul", Box::new(move |acc: i32| acc * acc));
    m.insert("div", Box::new(move |acc: i32| acc / acc));

    let mut res = 0;
    for argument in env::args() {
        let fnd = m.get(&argument);
        res = match fnd {
            None => res,
            Some(func) => func(res),
        }
    }
    println!("a = {}", res);
}

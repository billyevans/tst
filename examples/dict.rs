extern crate tst;

use std::env;
use self::tst::tst::TST;
use std::io;
use std::fs::File;
use std::io::prelude::*;


fn match_prefix(m: &TST<u8>, prefix: &str) {
    println!("match('{}'):", prefix);
    for (k, _) in m.prefix_iter(prefix) {
        println!("{}", k);
    }
}

fn load_dict(path: &str, m: &mut TST<u8>) -> io::Result<()> {
    let mut buffer = String::new();
    let mut file = try!(File::open(path));
    try!(file.read_to_string(&mut buffer));
    // TODO: Add shuffle sort of lines, for better spread
    for line in buffer.split('\n') {
        if line.len() > 0 {
            m.insert(line, 0);
        }
    }

    Ok(())
}

fn main() {
    let mut m = TST::<u8>::new();

    if env::args().count() < 3 {
        panic!("usage: {} <dict>  <prefix1> [<prefix2> ...]", &env::args().nth(0).unwrap());
    }
    // read dict
    load_dict(&env::args().nth(1).unwrap(), &mut m).unwrap();

    // print matched with prefix
    for argument in env::args() {
        match_prefix(&m, &argument);
    }
}

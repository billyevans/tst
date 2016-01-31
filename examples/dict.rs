extern crate tst;

use std::env;
use self::tst::TSTSet;
use std::io;
use std::fs::File;
use std::io::prelude::*;

fn match_prefix(set: &TSTSet, prefix: &str) {
    println!("match('{}'):", prefix);
    if prefix.len() > 0 {
        for k in set.prefix_iter(prefix) {
            println!("{}", k);
        }
    } else {
        for k in set.iter() {
            println!("{}", k);
        }
    }
}

fn load_dict(path: &str, set: &mut TSTSet) -> io::Result<()> {
    let mut buffer = String::new();
    let mut file = try!(File::open(path));
    try!(file.read_to_string(&mut buffer));
    // TODO: Add shuffle sort of lines, for better spread
    for line in buffer.split('\n') {
        if line.len() > 0 {
            set.insert(line);
        }
    }
    Ok(())
}

fn main() {
    if env::args().count() < 3 {
        panic!("usage: {} <dict>  <prefix1> [<prefix2> ...]",
               &env::args().nth(0).unwrap());
    }
    // read dict
    let mut set = TSTSet::new();
    load_dict(&env::args().nth(1).unwrap(), &mut set).unwrap();

    // print matched with prefix
    let mut args = env::args();
    args.next();
    args.next();
    for argument in args {
        match_prefix(&set, &argument);
    }
}

extern crate jemallocator;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

extern crate libc;
//extern {fn __rjem_je_stats_print (write_cb: extern fn (*const libc::c_void, *const libc::c_char), cbopaque: *const libc::c_void, opts: *const libc::c_char);}
extern fn write_cb (_: *mut libc::c_void, message: *const libc::c_char) {
    print! ("{}", String::from_utf8_lossy (unsafe {std::ffi::CStr::from_ptr (message as *const i8) .to_bytes()}));}
extern crate jemalloc_sys;

use std::env;
use tst::TSTSet;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use rand::thread_rng;
use rand::seq::SliceRandom;


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
    let mut file = File::open(path)?;
    file.read_to_string(&mut buffer)?;

    let mut v = vec![];
    for line in buffer.split('\n') {
        if line.len() > 0 {
            v.push(line);

        }
    }
    let mut rng = thread_rng();
    v.shuffle(&mut rng);

    for line in v.iter() {
        set.insert(line);
    }

    Ok(())
}

fn main() {
    if env::args().count() < 3 {
        panic!("usage: {} <dict> <prefix1> [<prefix2> ...]",
               &env::args().nth(0).unwrap());
    }
    // read dict
    let mut set = TSTSet::new();
    load_dict(&env::args().nth(1).unwrap(), &mut set).unwrap();
    //TODO: use flag
    {unsafe {jemalloc_sys::malloc_stats_print (write_cb, std::ptr::null_mut(), std::ptr::null())};}
    // print matched with prefix
    let mut args = env::args();
    args.next();
    args.next();
    for argument in args {
        match_prefix(&set, &argument);
    }
}

extern crate tst;

#[cfg(test)]
use tst::tst::*;
#[test]
fn create_root() {
	let m = TST::<i32>::new();
	assert_eq!(0, m.size());
}

#[test]
fn put() {
	let mut m = TST::<i32>::new();

	m.put("abc", &13);
	assert_eq!(1, m.size());
}

#[test]
fn get() {
	let mut m = TST::<i32>::new();

	m.put("abc", &13);
	assert_eq!(Some(&13), m.get("abc"));
}

#[test]
fn get_none() {
	let mut m = TST::<i32>::new();

	m.put("abc", &13);
	assert_eq!(None, m.get("abcd"));
	assert_eq!(None, m.get(""));
}

#[test]
fn put_few() {
	let mut m = TST::<i32>::new();

	m.put("abcde", &13);
	m.put("abcdf", &14);
	m.put("abcdg", &15);
	assert_eq!(3, m.size());

	assert_eq!(Some(&13), m.get("abcde"));
	assert_eq!(Some(&14), m.get("abcdf"));
	assert_eq!(Some(&15), m.get("abcdg"));
	assert_eq!(None, m.get("abcdh"));
}

#[test]
fn replace() {
	let mut m = TST::<i32>::new();

	m.put("abcde", &13);
	m.put("abcde", &1);
	assert_eq!(1, m.size());

	assert_eq!(Some(&1), m.get("abcde"));
}
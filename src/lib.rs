//! Ternary search trie (TST) container.

/// - Create a `TST` containing a given list of elements:
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate tst;
/// # fn main() {
/// let m = tstmap!{
///     "b" => 2, "a" => -1, "c" => 3,
/// };
///
/// assert_eq!(3, m.len());
/// assert_eq!(m["a"], -1);
/// assert_eq!(m["b"], 2);
/// assert_eq!(m["c"], 3);
/// # }
/// ```
#[macro_export]
macro_rules! tstmap {
    () => {{
        $crate::TSTMap::new()
    }};
    // trailing comma case
    ($($key:expr => $value:expr,)+) => (tstmap!($($key => $value),+));
    ($( $key: expr => $val: expr ),*) => {{
        let mut m = $crate::TSTMap::new();
        $(
            m.insert($key, $val);
        )*
        m
    }};
}

pub use tst_map::TSTMap;

/// TST container map implementation.
mod node;
mod map;

pub mod tst_map {
    pub use map::*;
}

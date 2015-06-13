

/// - Create a `TST` containing a given list of elements:
///
/// # Example
///
/// ```
/// #[macro_use] 
/// extern crate tst;
/// # fn main() {
/// let m = tstmap!{
/// 	"b" => 2, "a" => -1, "c" => 3,
///	};
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
        $crate::tst::TST::new()
    }};
    // trailing comma case
    ($($key:expr => $value:expr,)+) => (tstmap!($($key => $value),+));
    ($( $key: expr => $val: expr ),*) => {{
        let mut m = $crate::tst::TST::new();
        $(
            m.insert($key, $val); 
        )*
        m
    }};
}

pub use tst::TST;
pub mod tst;
// FIXME: uncomment, when test became stable
// mod bench;
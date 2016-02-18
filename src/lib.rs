//! A macro like `try!`, but turns a `Result` into an `Option`, mainly for use in a `filter_map`.

/// A macro like `try!` but turns a `Result` into an `Option`, mainly for use in a `filter_map`.
/// Provides early return of Errors like `try!` does in `map`.
///
/// # Example
///
/// ```
/// #[macro_use]
/// extern crate try_into_opt;
///
/// const MAX_NUM: usize = 11usize;
///
/// fn num_range_filter(v: &[usize]) -> Result<Vec<usize>, String> {
///     v.iter()
///      .filter_map(|i| {
///          let i = try_into_opt!(validate_num(*i));
///
///          // do some filtering...
///          if i == 5 {
///              None
///          } else {
///              Some(Ok(i))
///          }
///      })
///      .collect()
/// }
///
/// fn validate_num(i: usize) -> Result<usize, String> {
///     if i > MAX_NUM {
///         Err(format!("{} is larger than the allowed max of {}", i, MAX_NUM))
///     } else {
///         Ok(i)
///     }
/// }
///
/// fn main() {
///     let nums = num_range_filter(&[1, 2, 3, 10, 12, 0]);
///     assert_eq!(nums.is_err(), true);
///
///     let nums = num_range_filter(&[1, 2, 5, 3, 5, 10, 0, 5]).unwrap();
///     assert_eq!(nums, [1, 2, 3, 10, 0]);
/// }
/// ```
#[macro_export]
macro_rules! try_into_opt {
    ($e:expr) =>(
        match $e {
            ::std::result::Result::Ok(val) => val,
            ::std::result::Result::Err(err) => return Some(::std::result::Result::Err(::std::convert::From::from(err)))
        }
    )
}

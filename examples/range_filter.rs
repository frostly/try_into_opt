#![deny(missing_debug_implementations,
        missing_docs,
        trivial_casts,
        trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces,
        unused_qualifications,
        unused_variables)]
#![cfg_attr(feature = "nightly-testing", allow(unstable_features))]
#![cfg_attr(feature = "nightly-testing", feature(plugin))]
#![cfg_attr(feature = "nightly-testing", plugin(clippy))]
#![cfg_attr(feature = "nightly-testing", deny(clippy))]

//! Macro example

#[macro_use]
extern crate try_into_opt;

const MAX_NUM: usize = 11usize;

fn num_range_filter(v: &[usize]) -> Result<Vec<usize>, String> {
    v.iter()
     .filter_map(|i| {
         let i = try_into_opt!(validate_num(*i));

         // do some filtering...
         if i == 5 {
             None
         } else {
             Some(Ok(i))
         }
     })
     .collect()
}

fn validate_num(i: usize) -> Result<usize, String> {
    if i > MAX_NUM {
        Err(format!("{} is larger than the allowed max of {}", i, MAX_NUM))
    } else {
        Ok(i)
    }
}

fn main() {
    let nums = num_range_filter(&[1, 2, 3, 10, 12, 0]);
    assert_eq!(nums.is_err(), true);

    let nums = num_range_filter(&[1, 2, 5, 3, 5, 10, 0, 5]).unwrap();
    assert_eq!(nums, [1, 2, 3, 10, 0]);
}

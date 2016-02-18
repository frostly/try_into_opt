# `try_into_opt!`
[![Travis Build Status](https://img.shields.io/travis/frostly/try_into_opt.svg)](https://travis-ci.org/frostly/try_into_opt)
[![Documentation](https://img.shields.io/badge/docs-latest-C9893D.svg)](https://open.frostly.com/try_into_opt)
[![Coverage Status](https://img.shields.io/coveralls/frostly/try_into_opt.svg)](https://coveralls.io/github/frostly/try_into_opt?branch=master)
[![crates.io](https://img.shields.io/crates/v/try_into_opt.svg)](https://crates.io/crates/try_into_opt)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT)
[![Apache licensed](https://img.shields.io/badge/license-Apache-blue.svg)](./LICENSE-APACHE)

# Overview

Like `try!`, but turns a `Result` into an `Option`, mainly for use in a `filter_map`. Provides
early return for errors like `try!` does in `map`.


`try_into_opt!` is different from [try_opt!](https://github.com/crumblingstatue/try_opt), which
provides early return for functions that return an `Option`. `try_into_opt` operates on functions
that return a `Result` but your code requires an option. This scenario may come up when doing IO
that can fail, but if it succeeds, needs to be filtered in some way.

# Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
try_into_opt = "0.1.0"
```

And to your crate:

```rust,ignore
#[macro_use]
extern crate try_into_opt;
```

# Example

```rust
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
```

For more details, head over to the [documentation](https://open.frostly.com/try_into_opt).

# License

This library is distributed under similar terms to Rust: dual-licensed under the MIT license and the
Apache license (version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and [COPYRIGHT](COPYRIGHT) for
details.

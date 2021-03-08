# TryDefault

[![Current Crates.io Version](https://img.shields.io/crates/v/try_default.svg)](https://crates.io/crates/try_default)

A _'try'_ version of `Default`, where it returns an `Option::Some` wrapping the default value when present.
It returns `Option::None` when there is no `Default` available.

You can in theory call this on any type to retrive it's Default, if there is one!

```
use ::try_default::TryDefault;

// Set to `Some(0)`.
let default_num = <u32>::try_default();

// Set to `None`, as `::std::fs::File` has no `Default`.
let default_file = <::std::fs::File>::try_default();
```

## Notes

 * Requires nightly for `specialization` feature.
 * Is possibly the world's most pointless crate. But it's here if you need it!

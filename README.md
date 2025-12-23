# serde_derive_rescript

📝 This is a fork of [`serde_derive`](https://github.com/serde-rs/serde) crate that patches the original macros to work with ReScript's [`sury`](https://github.com/DZakh/sury) ppx.

The key differences are:
1. camelCase by default
2. Serialization/deserialization of enums matches the way ReScript handles tagged unions:

```rust
 #[derive(SerializeDto, DeserializeDto)]
enum T {
  A,
  B { enum_field: usize },
}

T::A // -> "A"
T::B { enum_field: 42 } // -> { "TAG": "B", "enumField": 42 }
```

This way, serde on both ends is pretty much effortless:

```rust
// Rust
#[derive(SerializeDto, DeserializeDto)]
enum NameError {
    Empty,
    TooShort { min: usize, max: usize },
}
```

```rescript
// ReScript
module NameError = {
    @schema
    type t =
        | Empty
        | TooShort({min: int, max: int})
}
```

---

#### Serde Version

Starting from serde 1.0.225, serde uses versioned private modules (`__private225`, `__private226`, etc.) that must match between serde and serde_derive. Since this is a fork with its own versioning, we mirror serde's version to maintain compatibility.

You must use the matching serde version:

```toml
[dependencies]
serde = "1.0.228"

# Option 1: Latest on main (gets bug fixes automatically)
serde_derive_rescript = { git = "https://github.com/alex35mil/serde_derive_rescript", branch = "main" }

# Option 2: Pinned to specific commit
serde_derive_rescript = { git = "https://github.com/alex35mil/serde_derive_rescript", rev = "abc1234" }
```

Cargo will fail with a version conflict if serde versions don't match.

---

I didn't publish this crate anywhere. I'm using it in my project from this repo. I can push it to crates if there's any interest - let me know in the issues.

---

Base commit: [`d17902059e77e371d8a7f83ff403f9e760b70f45`](https://github.com/serde-rs/serde/commit/d17902059e77e371d8a7f83ff403f9e760b70f45)

---

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Serde by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>

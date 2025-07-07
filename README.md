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

I didn't publish this crate anywhere. I'm using it in my project from this repo. I can push it to crates if there's any interest - let me know in the issues.

---

Base commit: [`babafa54d283fb087fa94f50a2cf82fa9e582a7c`](https://github.com/serde-rs/serde/commit/babafa54d283fb087fa94f50a2cf82fa9e582a7c)

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

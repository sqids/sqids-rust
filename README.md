# [Sqids Rust](https://sqids.org/rust)

[![Latest version](https://img.shields.io/crates/v/sqids.svg)](https://crates.io/crates/sqids)
[![Github Actions](https://img.shields.io/github/actions/workflow/status/sqids/sqids-rust/tests.yml)](https://github.com/sqids/sqids-rust/actions)
[![Docs](https://docs.rs/sqids/badge.svg)](https://docs.rs/sqids/latest/sqids/)
[![Downloads](https://img.shields.io/crates/d/sqids)](https://crates.io/crates/sqids)

[Sqids](https://sqids.org/rust) (*pronounced "squids"*) is a small library that lets you **generate unique IDs from numbers**. It's good for link shortening, fast & URL-safe ID generation and decoding back into numbers for quicker database lookups.

Features:

- **Encode multiple numbers** - generate short IDs from one or several non-negative numbers
- **Quick decoding** - easily decode IDs back into numbers
- **Unique IDs** - generate unique IDs by shuffling the alphabet once
- **ID padding** - provide minimum length to make IDs more uniform
- **URL safe** - auto-generated IDs do not contain common profanity
- **Randomized output** - Sequential input provides nonconsecutive IDs
- **Many implementations** - Support for [40+ programming languages](https://sqids.org/)

## 🧰 Use-cases

Good for:

- ✅ Generating IDs for public URLs (eg: link shortening)
- ✅ Generating IDs for internal systems (eg: event tracking)
- ✅ Decoding for quicker database lookups (eg: by primary keys)

Not good for:

- ❌ Sensitive data (this is not an encryption library)
- ❌ User IDs (can be decoded revealing user count)

## 🚀 Getting started

Add using cargo:

```bash
cargo add sqids
```

## 👩‍💻 Examples

Simple encode & decode:

```rust
let sqids = Sqids::default();
let id = sqids.encode(&[1, 2, 3])?; // "8QRLaD"
let numbers = sqids.decode(&id); // [1, 2, 3]
```

> **Note**
> 🚧 Because of the algorithm's design, **multiple IDs can decode back into the same sequence of numbers**. If it's important to your design that IDs are canonical, you have to manually re-encode decoded numbers and check that the generated ID matches.

Randomize IDs by providing a custom alphabet:

```rust
let sqids = Sqids::new(Some(Options::new(
  Some("FxnXM1kBN6cuhsAvjW3Co7l2RePyY8DwaU04Tzt9fHQrqSVKdpimLGIJOgb5ZE".to_string()),
  None,
  None,
)))?;
let id = sqids.encode(&[1, 2, 3])?; // "B5aMa3"
let numbers = sqids.decode(&id); // [1, 2, 3]
```

Enforce a *minimum* length for IDs:

```rust
let sqids = Sqids::new(Some(Options::new(
  None,
  Some(10),
  None,
)))?;
let id = sqids.encode(&[1, 2, 3])?; // "75JT1cd0dL"
let numbers = sqids.decode(&id); // [1, 2, 3]
```

Prevent specific words from appearing anywhere in the auto-generated IDs:

```rust
let sqids = Sqids::new(Some(Options::new(
  None,
  None,
  Some(HashSet::from(["word1".to_string(), "word2".to_string()])),
)))?;
let id = sqids.encode(&[1, 2, 3])?; // "8QRLaD"
let numbers = sqids.decode(&id); // [1, 2, 3]
```

## 📝 License

[MIT](LICENSE)

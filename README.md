# [Sqids Rust](https://sqids.org/rust)

[![Github Actions](https://img.shields.io/github/actions/workflow/status/sqids/sqids-rust/tests.yml)](https://github.com/sqids/sqids-rust/actions)

Sqids (pronounced "squids") is a small library that lets you generate YouTube-looking IDs from numbers. It's good for link shortening, fast & URL-safe ID generation and decoding back into numbers for quicker database lookups.

## Getting started

Add using cargo:

```bash
cargo add sqids
```

## Examples

Simple encode & decode:

```rust
let sqids = Sqids::default();
let id = sqids.encode(&[1, 2, 3])?; // "8QRLaD"
let numbers = sqids.decode(id); // [1, 2, 3]
```

Randomize IDs by providing a custom alphabet:

```rust
let sqids = Sqids::new(Some(Options::new(
  Some("FxnXM1kBN6cuhsAvjW3Co7l2RePyY8DwaU04Tzt9fHQrqSVKdpimLGIJOgb5ZE".to_string()),
  None,
  None,
)))?;
let id = sqids.encode(&[1, 2, 3])?; // "B5aMa3"
let numbers = sqids.decode(id); // [1, 2, 3]
```

Enforce a *minimum* length for IDs:

```rust
let sqids = Sqids::new(Some(Options::new(
  None,
  Some(10),
  None,
)))?;
let id = sqids.encode(&[1, 2, 3])?; // "75JT1cd0dL"
let numbers = sqids.decode(id); // [1, 2, 3]
```

Prevent specific words from appearing anywhere in the auto-generated IDs:

```rust
let sqids = Sqids::new(Some(Options::new(
  None,
  None,
  Some(HashSet::from(["word1".to_string(), "word2".to_string()])),
)))?;
let id = sqids.encode(&[1, 2, 3])?; // "8QRLaD"
let numbers = sqids.decode(id); // [1, 2, 3]
```

## License

[MIT](LICENSE)

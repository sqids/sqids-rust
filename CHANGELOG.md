# CHANGELOG

**v0.4.2:**
- Fix u64 overflow panic [[PR #5](https://github.com/sqids/sqids-rust/pull/7)]
- Cargo update
- `cargo deny` update

**v0.4.1:**
- Derive `Clone` trait [[PR #6](https://github.com/sqids/sqids-rust/pull/6)]
- Cargo update

**v0.4.0:**
- Introduced `Sqids::builder()` [[PR #5](https://github.com/sqids/sqids-rust/pull/5)]
- Cargo update
- Docs cleanup

**v0.3.1:**
- Improvement: impl error for Error [[PR #3](https://github.com/sqids/sqids-rust/pull/3)]
- Using `thiserror`
- Cargo update

**v0.3.0:** **⚠️ BREAKING CHANGE**
- **Breaking change**: IDs change. Algorithm has been fine-tuned for better performance [[Issue #11](https://github.com/sqids/sqids-spec/issues/11)]
- `alphabet` cannot contain multibyte characters
- `min_length` was changed from `usize` to `u8`
- Max blocklist re-encoding attempts has been capped at the length of the alphabet - 1
- Minimum alphabet length has changed from 5 to 3
- `min_value()` and `max_value()` functions have been removed

**v0.2.1:**
- Bug fix: spec update (PR #7): blocklist filtering in uppercase-only alphabet [[PR #7](https://github.com/sqids/sqids-spec/pull/7)]
- Updating Github Actions to use stable toolchain instead of nightly
- Cargo update

**v0.2.0:**
- Bug fix: test for decoding an invalid ID with a repeating reserved character
- Cargo update

**v0.1.1:**
- Initial implementation of [the spec](https://github.com/sqids/sqids-spec)
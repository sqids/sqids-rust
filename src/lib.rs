#![warn(missing_docs)]
#![allow(clippy::tabs_in_doc_comments)]

#![doc = include_str!("../README.md")]

// Make the link to the LICENSE in README.md work.
#[cfg(doc)]
#[doc = include_str!("../LICENSE")]
///
/// ---
/// **Note**: This is the crate's license and not an actual item.
pub const LICENSE: () = ();

use std::{cmp::min, collections::HashSet, result};

use derive_builder::Builder;
use thiserror::Error;

/// sqids Error type.
#[derive(Error, Debug, Eq, PartialEq)]
pub enum Error {
	/// Alphabet cannot contain multibyte characters
	///
	/// ```
	/// # use sqids::{Sqids, Error};
	/// let error = Sqids::builder().alphabet("☃️🦀🔥".chars().collect()).build().unwrap_err();
	/// assert_eq!(error, Error::AlphabetMultibyteCharacters);
	/// ```
	#[error("Alphabet cannot contain multibyte characters")]
	AlphabetMultibyteCharacters,
	/// Alphabet length must be at least 3
	///
	/// ```
	/// # use sqids::{Sqids, Error};
	/// let error = Sqids::builder().alphabet("ab".chars().collect()).build().unwrap_err();
	/// assert_eq!(error, Error::AlphabetLength);
	/// ```
	#[error("Alphabet length must be at least 3")]
	AlphabetLength,
	/// Alphabet must contain unique characters
	///
	/// ```
	/// # use sqids::{Sqids, Error};
	/// let error = Sqids::builder().alphabet("aba".chars().collect()).build().unwrap_err();
	/// assert_eq!(error, Error::AlphabetUniqueCharacters);
	/// ```
	#[error("Alphabet must contain unique characters")]
	AlphabetUniqueCharacters,
	/// Reached max attempts to re-generate the ID
	///
	/// ```
	/// # use sqids::{Sqids, Error};
	/// let sqids = Sqids::builder()
	/// 	.alphabet("abc".chars().collect())
	/// 	.min_length(3)
	/// 	.blocklist(["aac".to_string(), "bba".to_string(), "ccb".to_string()].into())
	/// 	.build()
	/// 	.unwrap();
	/// let error = sqids.encode(&[1]).unwrap_err();
	/// assert_eq!(error, Error::BlocklistMaxAttempts);
	/// ```
	#[error("Reached max attempts to re-generate the ID")]
	BlocklistMaxAttempts,
}

/// type alias for Result<T, Error>
pub type Result<T> = result::Result<T, Error>;

/// The default alphabet used when none is given when creating a [Sqids].
pub const DEFAULT_ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

/// Returns the default blocklist when none is given when creating a [Sqids].
pub fn default_blocklist() -> HashSet<String> {
	serde_json::from_str(include_str!("blocklist.json")).unwrap()
}

/// Options for creating a [Sqids].
#[derive(Debug)]
pub struct Options {
	/// The [Sqids] alphabet.
	pub alphabet: String,
	/// The minimum length of a sqid.
	pub min_length: u8,
	/// Blocklist. When creating a sqid [Sqids] will try to avoid generating a string that begins
	/// with one of these.
	pub blocklist: HashSet<String>,
}

impl Options {
	/// Create an [Options] object.
	pub fn new(
		alphabet: Option<String>,
		min_length: Option<u8>,
		blocklist: Option<HashSet<String>>,
	) -> Self {
		let mut options = Options::default();

		if let Some(alphabet) = alphabet {
			options.alphabet = alphabet;
		}
		if let Some(min_length) = min_length {
			options.min_length = min_length;
		}
		if let Some(blocklist) = blocklist {
			options.blocklist = blocklist;
		}

		options
	}
}

impl Default for Options {
	fn default() -> Self {
		Options {
			alphabet: DEFAULT_ALPHABET.to_string(),
			min_length: 0,
			blocklist: default_blocklist(),
		}
	}
}

/// A generator for sqids.
#[derive(Debug, Builder)]
#[builder(build_fn(skip, error = "Error"), pattern = "owned")]
pub struct Sqids {
	/// The alphabet that is being used when generating sqids.
	alphabet: Vec<char>,
	/// The minimum length of a sqid.
	min_length: u8,
	/// Blocklist. When creating a sqid strings that begins
	/// with one of these will be avoided.
	blocklist: HashSet<String>,
}

impl Default for Sqids {
	fn default() -> Self {
		Self::builder().build().unwrap()
	}
}

impl SqidsBuilder {
	/// Create a [SqidsBuilder].
	pub fn new() -> Self {
		Self::default()
	}

	/// Build a [Sqids] object.
	pub fn build(self) -> Result<Sqids> {
		let alphabet: Vec<char> =
			self.alphabet.unwrap_or_else(|| DEFAULT_ALPHABET.chars().collect());

		for c in alphabet.iter() {
			if c.len_utf8() > 1 {
				return Err(Error::AlphabetMultibyteCharacters);
			}
		}

		if alphabet.len() < 3 {
			return Err(Error::AlphabetLength);
		}

		let unique_chars: HashSet<char> = alphabet.iter().cloned().collect();
		if unique_chars.len() != alphabet.len() {
			return Err(Error::AlphabetUniqueCharacters);
		}

		let lowercase_alphabet: Vec<char> =
			alphabet.iter().map(|c| c.to_ascii_lowercase()).collect();
		let filtered_blocklist: HashSet<String> = self
			.blocklist
			.unwrap_or_else(default_blocklist)
			.iter()
			.filter_map(|word| {
				let word = word.to_lowercase();
				if word.len() >= 3 && word.chars().all(|c| lowercase_alphabet.contains(&c)) {
					Some(word)
				} else {
					None
				}
			})
			.collect();

		Ok(Sqids {
			alphabet: Sqids::shuffle(&alphabet),
			min_length: self.min_length.unwrap_or(0),
			blocklist: filtered_blocklist,
		})
	}
}

impl Sqids {
	/// Create a [Sqids] from [Options].
	pub fn new(options: Option<Options>) -> Result<Self> {
		let options = options.unwrap_or_default();
		Self::builder()
			.min_length(options.min_length)
			.alphabet(options.alphabet.chars().collect())
			.blocklist(options.blocklist)
			.build()
	}

	/// Create a [SqidsBuilder].
	pub fn builder() -> SqidsBuilder {
		SqidsBuilder::default()
	}

	/// Generate a sqid from a slice of numbers.
	///
	/// When an sqid is generated it is checked against the [SqidsBuilder::blocklist]. When a
	/// blocked word is encountered another attempt is made by shifting the alphabet.
	/// When the alphabet is exhausted and all possible sqids for this input are blocked
	/// [Error::BlocklistMaxAttempts] is returned.
	pub fn encode(&self, numbers: &[u64]) -> Result<String> {
		if numbers.is_empty() {
			return Ok(String::new());
		}

		self.encode_numbers(numbers, 0)
	}

	/// Decode a sqid into a vector of numbers. When an invalid sqid is encountered an empty vector
	/// is returned.
	pub fn decode(&self, id: &str) -> Vec<u64> {
		let mut ret = Vec::new();

		if id.is_empty() {
			return ret;
		}

		let alphabet_chars: HashSet<char> = self.alphabet.iter().cloned().collect();
		if !id.chars().all(|c| alphabet_chars.contains(&c)) {
			return ret;
		}

		let prefix = id.chars().next().unwrap();
		let offset = self.alphabet.iter().position(|&c| c == prefix).unwrap();
		let mut alphabet: Vec<char> =
			self.alphabet.iter().cycle().skip(offset).take(self.alphabet.len()).copied().collect();

		alphabet = alphabet.into_iter().rev().collect();

		let mut id = id[1..].to_string();

		while !id.is_empty() {
			let separator = alphabet[0];

			let chunks: Vec<&str> = id.split(separator).collect();
			if !chunks.is_empty() {
				if chunks[0].is_empty() {
					return ret;
				}

				let alphabet_without_separator: Vec<char> =
					alphabet.iter().copied().skip(1).collect();
				ret.push(self.to_number(chunks[0], &alphabet_without_separator));

				if chunks.len() > 1 {
					alphabet = Self::shuffle(&alphabet);
				}
			}

			id = chunks[1..].join(&separator.to_string());
		}

		ret
	}

	fn encode_numbers(&self, numbers: &[u64], increment: usize) -> Result<String> {
		if increment > self.alphabet.len() {
			return Err(Error::BlocklistMaxAttempts);
		}

		let mut offset = numbers.iter().enumerate().fold(numbers.len(), |a, (i, &v)| {
			self.alphabet[v as usize % self.alphabet.len()] as usize + i + a
		}) % self.alphabet.len();

		offset = (offset + increment) % self.alphabet.len();

		let mut alphabet: Vec<char> =
			self.alphabet.iter().cycle().skip(offset).take(self.alphabet.len()).copied().collect();

		let prefix = alphabet[0];

		alphabet = alphabet.into_iter().rev().collect();

		let mut ret: Vec<String> = vec![prefix.to_string()];

		for (i, &num) in numbers.iter().enumerate() {
			ret.push(self.to_id(num, &alphabet[1..]));

			if i < numbers.len() - 1 {
				ret.push(alphabet[0].to_string());
				alphabet = Self::shuffle(&alphabet);
			}
		}

		let mut id = ret.join("");

		if self.min_length as usize > id.len() {
			id += &alphabet[0].to_string();

			while self.min_length as usize - id.len() > 0 {
				alphabet = Self::shuffle(&alphabet);

				let slice_len = min(self.min_length as usize - id.len(), alphabet.len());
				let slice: Vec<char> = alphabet.iter().take(slice_len).cloned().collect();

				id += &slice.iter().collect::<String>();
			}
		}

		if self.is_blocked_id(&id) {
			id = self.encode_numbers(numbers, increment + 1)?;
		}

		Ok(id)
	}

	fn to_id(&self, num: u64, alphabet: &[char]) -> String {
		let mut id = Vec::new();
		let mut result = num;

		loop {
			let idx = (result % alphabet.len() as u64) as usize;
			id.insert(0, alphabet[idx]);
			result /= alphabet.len() as u64;

			if result == 0 {
				break;
			}
		}

		id.into_iter().collect()
	}

	fn to_number(&self, id: &str, alphabet: &[char]) -> u64 {
		let mut result = 0;

		for c in id.chars() {
			let idx = alphabet.iter().position(|&x| x == c).unwrap();
			result = result * alphabet.len() as u64 + idx as u64;
		}

		result
	}

	fn shuffle(alphabet: &[char]) -> Vec<char> {
		let mut chars: Vec<char> = alphabet.to_vec();

		for i in 0..(chars.len() - 1) {
			let j = chars.len() - 1 - i;
			let r = (i as u32 * j as u32 + chars[i] as u32 + chars[j] as u32) % chars.len() as u32;
			chars.swap(i, r as usize);
		}

		chars
	}

	fn is_blocked_id(&self, id: &str) -> bool {
		let id = id.to_lowercase();

		for word in &self.blocklist {
			if word.len() <= id.len() {
				if id.len() <= 3 || word.len() <= 3 {
					if id == *word {
						return true;
					}
				} else if word.chars().any(|c| c.is_ascii_digit()) {
					if id.starts_with(word) || id.ends_with(word) {
						return true;
					}
				} else if id.contains(word) {
					return true;
				}
			}
		}

		false
	}
}

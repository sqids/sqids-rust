use derive_more::Display;
use std::collections::HashSet;
use std::result;

#[derive(Display, Debug)]
pub enum Error {
    #[display(fmt = "Alphabet length must be at least 5")]
    AlphabetLength,
    #[display(fmt = "Alphabet must contain unique characters")]
    AlphabetUniqueCharacters,
    #[display(fmt = "Minimum length has to be between {min} and {max}")]
    MinLength { min: usize, max: usize },
    #[display(fmt = "Encoding supports numbers between {min} and {max}")]
    EncodingRange { min: u64, max: u64 },
    #[display(fmt = "Ran out of range checking against the blocklist")]
    BlocklistOutOfRange,
}

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Options {
    alphabet: String,
    min_length: usize,
    blocklist: HashSet<String>,
}

impl Options {
    pub fn new(
        alphabet: Option<String>,
        min_length: Option<usize>,
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
            alphabet: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".to_string(),
            min_length: 0,
            blocklist: serde_json::from_str(include_str!("blocklist.json")).unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct Sqids {
    alphabet: Vec<char>,
    min_length: usize,
    blocklist: HashSet<String>,
}

impl Sqids {
    pub fn new(options: Option<Options>) -> Result<Self> {
        let options = options.unwrap_or_default();
        let alphabet: Vec<char> = options.alphabet.chars().collect();

        if alphabet.len() < 5 {
            return Err(Error::AlphabetLength);
        }

        let unique_chars: HashSet<char> = alphabet.iter().cloned().collect();
        if unique_chars.len() != alphabet.len() {
            return Err(Error::AlphabetUniqueCharacters);
        }

        let filtered_blocklist: HashSet<String> = options
            .blocklist
            .iter()
            .filter_map(|word| {
                let word = word.to_lowercase();
                if word.len() >= 3 && word.chars().all(|c| alphabet.contains(&c)) {
                    Some(word)
                } else {
                    None
                }
            })
            .collect();

        let mut sqids = Sqids {
            alphabet,
            min_length: options.min_length,
            blocklist: filtered_blocklist,
        };

        if options.min_length < sqids.min_value() as usize
            || options.min_length > options.alphabet.len()
        {
            return Err(Error::MinLength {
                min: sqids.min_value() as usize,
                max: options.alphabet.len(),
            });
        }

        sqids.alphabet = sqids.shuffle(&sqids.alphabet);
        Ok(sqids)
    }

    pub fn encode(&self, numbers: &[u64]) -> Result<String> {
        if numbers.is_empty() {
            return Ok(String::new());
        }

        let in_range_numbers: Vec<u64> = numbers
            .iter()
            .copied()
            .filter(|&n| n >= self.min_value() && n <= self.max_value())
            .collect();

        if in_range_numbers.len() != numbers.len() {
            return Err(Error::EncodingRange {
                min: self.min_value(),
                max: self.max_value(),
            });
        }

        self.encode_numbers(&in_range_numbers, false)
    }

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
        let mut alphabet: Vec<char> = self
            .alphabet
            .iter()
            .cycle()
            .skip(offset)
            .take(self.alphabet.len())
            .copied()
            .collect();

        let partition = alphabet[1];

        alphabet.remove(1);
        alphabet.remove(0);

        let mut id = id[1..].to_string();

        let partition_index = id.find(partition);
        if let Some(idx) = partition_index {
            if idx > 0 && idx < id.len() - 1 {
                id = id.split_off(idx + 1);
                alphabet = self.shuffle(&alphabet);
            }
        }

        while !id.is_empty() {
            let separator = alphabet[alphabet.len() - 1];
            let chunks: Vec<&str> = id.split(separator).collect();

            if !chunks.is_empty() {
                let alphabet_without_separator: Vec<char> =
                    alphabet.iter().copied().take(alphabet.len() - 1).collect();
                let num = self.to_number(chunks[0], &alphabet_without_separator);
                ret.push(num);

                if chunks.len() > 1 {
                    alphabet = self.shuffle(&alphabet);
                }
            }

            id = chunks[1..].join(&separator.to_string());
        }

        ret
    }

    pub fn min_value(&self) -> u64 {
        0
    }

    pub fn max_value(&self) -> u64 {
        u64::MAX
    }

    fn encode_numbers(&self, numbers: &[u64], partitioned: bool) -> Result<String> {
        let offset = numbers
            .iter()
            .enumerate()
            .fold(numbers.len(), |a, (i, &v)| {
                self.alphabet[v as usize % self.alphabet.len()] as usize + i + a
            })
            % self.alphabet.len();

        let mut alphabet: Vec<char> = self
            .alphabet
            .iter()
            .cycle()
            .skip(offset)
            .take(self.alphabet.len())
            .copied()
            .collect();

        let prefix = alphabet[0];
        let partition = alphabet[1];

        alphabet.remove(1);
        alphabet.remove(0);

        let mut ret: Vec<String> = vec![prefix.to_string()];

        for (i, &num) in numbers.iter().enumerate() {
            let alphabet_without_separator: Vec<char> =
                alphabet.iter().copied().take(alphabet.len() - 1).collect();
            ret.push(self.to_id(num, &alphabet_without_separator));

            if i < numbers.len() - 1 {
                let separator = alphabet[alphabet.len() - 1];

                if partitioned && i == 0 {
                    ret.push(partition.to_string());
                } else {
                    ret.push(separator.to_string());
                }

                alphabet = self.shuffle(&alphabet);
            }
        }

        let mut id = ret.join("");

        if self.min_length > id.len() {
            if !partitioned {
                let mut new_numbers = vec![0];
                new_numbers.extend_from_slice(numbers);
                id = self.encode_numbers(&new_numbers, true)?;
            }

            if self.min_length > id.len() {
                let mut new_id = id.clone();
                let alphabet_slice = &alphabet[..(self.min_length - id.len())];
                new_id.push_str(&alphabet_slice.iter().collect::<String>());
                new_id.push_str(&id[1..]);
                id = new_id;
            }
        }

        if self.is_blocked_id(&id) {
            let mut new_numbers;

            if partitioned {
                if numbers[0] + 1 > self.max_value() {
                    return Err(Error::BlocklistOutOfRange);
                } else {
                    new_numbers = numbers.to_vec();
                    new_numbers[0] += 1;
                }
            } else {
                new_numbers = vec![0];
                new_numbers.extend_from_slice(numbers);
            }

            id = self.encode_numbers(&new_numbers, true)?;
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

    fn shuffle(&self, alphabet: &[char]) -> Vec<char> {
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

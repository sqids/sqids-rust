use sqids::*;

#[test]
fn simple() {
	let sqids =
		Sqids::new(Some(Options::new(Some("0123456789abcdef".to_string()), None, None))).unwrap();

	let numbers = vec![1, 2, 3];
	let id = "489158";

	assert_eq!(sqids.encode(&numbers).unwrap(), id);
	assert_eq!(sqids.decode(id), numbers);
}

#[test]
fn short_alphabet() {
	let sqids = Sqids::new(Some(Options::new(Some("abc".to_string()), None, None))).unwrap();

	let numbers = vec![1, 2, 3];
	assert_eq!(sqids.decode(&sqids.encode(&numbers).unwrap()), numbers);
}

#[test]
fn long_alphabet() {
	let sqids = Sqids::new(Some(Options::new(
        Some("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_+|{}[];:'\"/?.>,<`~".to_string()),
        None,
        None,
    ))).unwrap();

	let numbers = vec![1, 2, 3];
	assert_eq!(sqids.decode(&sqids.encode(&numbers).unwrap()), numbers);
}

#[test]
fn multibyte_characters() {
	assert_eq!(
		Sqids::new(Some(Options::new(Some("ë1092".to_string()), None, None,))).err().unwrap(),
		Error::AlphabetMultibyteCharacters
	)
}

#[test]
fn repeating_alphabet_characters() {
	assert_eq!(
		Sqids::new(Some(Options::new(Some("aabcdefg".to_string()), None, None,))).err().unwrap(),
		Error::AlphabetUniqueCharacters
	)
}

#[test]
fn too_short_alphabet() {
	assert_eq!(
		Sqids::new(Some(Options::new(Some("ab".to_string()), None, None,))).err().unwrap(),
		Error::AlphabetLength
	)
}

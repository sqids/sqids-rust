use sqids::*;
use std::collections::HashSet;

#[test]
fn if_no_custom_blocklist_param_use_default_blocklist() {
	let sqids = Sqids::default();

	assert_eq!(sqids.decode("sexy"), vec![200044]);
	assert_eq!(sqids.encode(&[200044]).unwrap(), "d171vI");
}

#[test]
fn if_empty_blocklist_param_passed_dont_use_any_blocklist() {
	let sqids = Sqids::new(Some(Options::new(None, None, Some(HashSet::new())))).unwrap();

	assert_eq!(sqids.decode("sexy"), vec![200044]);
	assert_eq!(sqids.encode(&[200044]).unwrap(), "sexy");
}

#[test]
fn if_non_empty_blocklist_param_passed_use_only_that() {
	let sqids =
		Sqids::new(Some(Options::new(None, None, Some(HashSet::from(["AvTg".to_string()])))))
			.unwrap();

	// make sure we don't use the default blocklist
	assert_eq!(sqids.decode("sexy"), vec![200044]);
	assert_eq!(sqids.encode(&[200044]).unwrap(), "sexy");

	// make sure we are using the passed blocklist
	assert_eq!(sqids.decode("AvTg"), vec![100000]);
	assert_eq!(sqids.encode(&[100000]).unwrap(), "7T1X8k");
	assert_eq!(sqids.decode("7T1X8k"), vec![100000]);
}

#[test]
fn blocklist() {
	let sqids = Sqids::new(Some(Options::new(
		None,
		None,
		Some(HashSet::from([
			"8QRLaD".to_owned(), // normal result of 1st encoding, let's block that word on purpose
			"7T1cd0dL".to_owned(), // result of 2nd encoding
			"UeIe".to_owned(),   // result of 3rd encoding is `RA8UeIe7`, let's block a substring
			"imhw".to_owned(),   // result of 4th encoding is `WM3Limhw`, let's block the postfix
			"LfUQ".to_owned(),   // result of 4th encoding is `LfUQh4HN`, let's block the prefix
		])),
	)))
	.unwrap();

	assert_eq!(sqids.encode(&[1, 2, 3]).unwrap(), "TM0x1Mxz");
	assert_eq!(sqids.decode("TM0x1Mxz"), vec![1, 2, 3]);
}

#[test]
fn decoding_blocklist_words_should_still_work() {
	let sqids = Sqids::new(Some(Options::new(
		None,
		None,
		Some(HashSet::from([
			"8QRLaD".to_owned(),
			"7T1cd0dL".to_owned(),
			"RA8UeIe7".to_owned(),
			"WM3Limhw".to_owned(),
			"LfUQh4HN".to_owned(),
		])),
	)))
	.unwrap();

	assert_eq!(sqids.decode("8QRLaD"), vec![1, 2, 3]);
	assert_eq!(sqids.decode("7T1cd0dL"), vec![1, 2, 3]);
	assert_eq!(sqids.decode("RA8UeIe7"), vec![1, 2, 3]);
	assert_eq!(sqids.decode("WM3Limhw"), vec![1, 2, 3]);
	assert_eq!(sqids.decode("LfUQh4HN"), vec![1, 2, 3]);
}

#[test]
fn match_against_short_blocklist_word() {
	let sqids = Sqids::new(Some(Options::new(None, None, Some(HashSet::from(["pPQ".to_owned()])))))
		.unwrap();

	assert_eq!(sqids.decode(&sqids.encode(&[1000]).unwrap()), vec![1000]);
}

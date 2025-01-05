use sqids::*;
use std::collections::HashSet;

#[test]
fn if_no_custom_blocklist_param_use_default_blocklist() {
	let sqids = Sqids::default();

	assert_eq!(sqids.decode("aho1e"), vec![4572721]);
	assert_eq!(sqids.encode(&[4572721]).unwrap(), "JExTR");
}

#[test]
fn if_empty_blocklist_param_passed_dont_use_any_blocklist() {
	let sqids = Sqids::new(Some(Options::new(None, None, Some(HashSet::new())))).unwrap();

	assert_eq!(sqids.decode("aho1e"), vec![4572721]);
	assert_eq!(sqids.encode(&[4572721]).unwrap(), "aho1e");
}

#[test]
fn if_non_empty_blocklist_param_passed_use_only_that() {
	let sqids =
		Sqids::new(Some(Options::new(None, None, Some(HashSet::from(["ArUO".to_string()])))))
			.unwrap();

	// make sure we don't use the default blocklist
	assert_eq!(sqids.decode("aho1e"), vec![4572721]);
	assert_eq!(sqids.encode(&[4572721]).unwrap(), "aho1e");

	// make sure we are using the passed blocklist
	assert_eq!(sqids.decode("ArUO"), vec![100000]);
	assert_eq!(sqids.encode(&[100000]).unwrap(), "QyG4");
	assert_eq!(sqids.decode("QyG4"), vec![100000]);
}

#[test]
fn blocklist() {
	let sqids = Sqids::new(Some(Options::new(
		None,
		None,
		Some(HashSet::from([
			"JSwXFaosAN".to_owned(), /* normal result of 1st encoding, let's block that word on
			                          * purpose */
			"OCjV9JK64o".to_owned(), // result of 2nd encoding
			"rBHf".to_owned(),       /* result of 3rd encoding is `4rBHfOiqd3`, let's block a
			                          * substring */
			"79SM".to_owned(), // result of 4th encoding is `dyhgw479SM`, let's block the postfix
			"7tE6".to_owned(), // result of 4th encoding is `7tE6jdAHLe`, let's block the prefix
		])),
	)))
	.unwrap();

	assert_eq!(sqids.encode(&[1_000_000, 2_000_000]).unwrap(), "1aYeB7bRUt");
	assert_eq!(sqids.decode("1aYeB7bRUt"), vec![1_000_000, 2_000_000]);
}

#[test]
fn decoding_blocklist_words_should_still_work() {
	let sqids = Sqids::new(Some(Options::new(
		None,
		None,
		Some(HashSet::from([
			"86Rf07".to_owned(),
			"se8ojk".to_owned(),
			"ARsz1p".to_owned(),
			"Q8AI49".to_owned(),
			"5sQRZO".to_owned(),
		])),
	)))
	.unwrap();

	assert_eq!(sqids.decode("86Rf07"), vec![1, 2, 3]);
	assert_eq!(sqids.decode("se8ojk"), vec![1, 2, 3]);
	assert_eq!(sqids.decode("ARsz1p"), vec![1, 2, 3]);
	assert_eq!(sqids.decode("Q8AI49"), vec![1, 2, 3]);
	assert_eq!(sqids.decode("5sQRZO"), vec![1, 2, 3]);
}

#[test]
fn match_against_short_blocklist_word() {
	let sqids = Sqids::new(Some(Options::new(None, None, Some(HashSet::from(["pnd".to_owned()])))))
		.unwrap();

	assert_eq!(sqids.decode(&sqids.encode(&[1000]).unwrap()), vec![1000]);
}

#[test]
fn blocklist_filtering_in_constructor() {
	let sqids = Sqids::new(Some(Options::new(
		Some("ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string()),
		None,
		Some(HashSet::from(["sxnzkl".to_owned()])), /* lowercase blocklist in only-uppercase
		                                             * alphabet */
	)))
	.unwrap();

	let id = sqids.encode(&[1, 2, 3]).unwrap();
	let numbers = sqids.decode(&id);

	assert_eq!(id, "IBSHOZ".to_string()); // without blocklist, would've been "SXNZKL"
	assert_eq!(numbers, vec![1, 2, 3]);
}

#[test]
fn max_encoding_attempts() {
	let alphabet = "abc".to_string();
	let min_length = 3;
	let blocklist = HashSet::from(["cab".to_owned(), "abc".to_owned(), "bca".to_owned()]);

	let sqids = Sqids::new(Some(Options::new(
		Some(alphabet.clone()),
		Some(min_length),
		Some(blocklist.clone()),
	)))
	.unwrap();

	assert_eq!(min_length as usize, alphabet.len());
	assert_eq!(min_length as usize, blocklist.len());

	assert_eq!(sqids.encode(&[0]).err().unwrap(), Error::BlocklistMaxAttempts);
}

#[test]
fn specific_is_blocked_id_scenarios() {
	let sqids = Sqids::builder().blocklist(["hey".to_string()].into()).build().unwrap();
	assert_eq!(sqids.encode(&[100]).unwrap(), "86u".to_string());

	let sqids = Sqids::builder().blocklist(["86u".to_string()].into()).build().unwrap();
	assert_eq!(sqids.encode(&[100]).unwrap(), "sec".to_string());

	let sqids = Sqids::builder().blocklist(["vFo".to_string()].into()).build().unwrap();
	assert_eq!(sqids.encode(&[1_000_000]).unwrap(), "gMvFo".to_string());

	let sqids = Sqids::builder().blocklist(["lP3i".to_string()].into()).build().unwrap();
	assert_eq!(sqids.encode(&[100, 202, 303, 404]).unwrap(), "oDqljxrokxRt".to_string());

	let sqids = Sqids::builder().blocklist(["1HkYs".to_string()].into()).build().unwrap();
	assert_eq!(sqids.encode(&[100, 202, 303, 404]).unwrap(), "oDqljxrokxRt".to_string());

	let sqids = Sqids::builder().blocklist(["0hfxX".to_string()].into()).build().unwrap();
	assert_eq!(
		sqids.encode(&[101, 202, 303, 404, 505, 606, 707]).unwrap(),
		"862REt0hfxXVdsLG8vGWD".to_string()
	);

	let sqids = Sqids::builder().blocklist(["hfxX".to_string()].into()).build().unwrap();
	assert_eq!(
		sqids.encode(&[101, 202, 303, 404, 505, 606, 707]).unwrap(),
		"seu8n1jO9C4KQQDxdOxsK".to_string()
	);
}

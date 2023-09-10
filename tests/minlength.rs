use sqids::*;

#[test]
fn simple() {
	let sqids =
		Sqids::new(Some(Options::new(None, Some(Options::default().alphabet.len() as u8), None)))
			.unwrap();

	let numbers = vec![1, 2, 3];
	let id = "86Rf07xd4zBmiJXQG6otHEbew02c3PWsUOLZxADhCpKj7aVFv9I8RquYrNlSTM".to_owned();

	assert_eq!(sqids.encode(&numbers).unwrap(), id);
	assert_eq!(sqids.decode(&id), numbers);
}

#[test]
fn incremental() {
	let numbers = [1, 2, 3];
	let alphabet_length = Options::default().alphabet.len() as u8;

	let map = vec![
		(6 as u8, "86Rf07".to_owned()),
		(7, "86Rf07x".to_owned()),
		(8, "86Rf07xd".to_owned()),
		(9, "86Rf07xd4".to_owned()),
		(10, "86Rf07xd4z".to_owned()),
		(11, "86Rf07xd4zB".to_owned()),
		(12, "86Rf07xd4zBm".to_owned()),
		(13, "86Rf07xd4zBmi".to_owned()),
		(
			alphabet_length + 0,
			"86Rf07xd4zBmiJXQG6otHEbew02c3PWsUOLZxADhCpKj7aVFv9I8RquYrNlSTM".to_owned(),
		),
		(
			alphabet_length + 1,
			"86Rf07xd4zBmiJXQG6otHEbew02c3PWsUOLZxADhCpKj7aVFv9I8RquYrNlSTMy".to_owned(),
		),
		(
			alphabet_length + 2,
			"86Rf07xd4zBmiJXQG6otHEbew02c3PWsUOLZxADhCpKj7aVFv9I8RquYrNlSTMyf".to_owned(),
		),
		(
			alphabet_length + 3,
			"86Rf07xd4zBmiJXQG6otHEbew02c3PWsUOLZxADhCpKj7aVFv9I8RquYrNlSTMyf1".to_owned(),
		),
	];

	for (min_length, id) in map {
		let sqids = Sqids::new(Some(Options::new(None, Some(min_length), None))).unwrap();

		assert_eq!(sqids.encode(&numbers).unwrap(), id);
		assert_eq!(sqids.encode(&numbers).unwrap().len(), min_length as usize);
		assert_eq!(sqids.decode(&id), numbers);
	}
}

#[test]
fn incremental_numbers() {
	let sqids =
		Sqids::new(Some(Options::new(None, Some(Options::default().alphabet.len() as u8), None)))
			.unwrap();

	let ids = vec![
		("SvIzsqYMyQwI3GWgJAe17URxX8V924Co0DaTZLtFjHriEn5bPhcSkfmvOslpBu".to_owned(), vec![0, 0]),
		("n3qafPOLKdfHpuNw3M61r95svbeJGk7aAEgYn4WlSjXURmF8IDqZBy0CT2VxQc".to_owned(), vec![0, 1]),
		("tryFJbWcFMiYPg8sASm51uIV93GXTnvRzyfLleh06CpodJD42B7OraKtkQNxUZ".to_owned(), vec![0, 2]),
		("eg6ql0A3XmvPoCzMlB6DraNGcWSIy5VR8iYup2Qk4tjZFKe1hbwfgHdUTsnLqE".to_owned(), vec![0, 3]),
		("rSCFlp0rB2inEljaRdxKt7FkIbODSf8wYgTsZM1HL9JzN35cyoqueUvVWCm4hX".to_owned(), vec![0, 4]),
		("sR8xjC8WQkOwo74PnglH1YFdTI0eaf56RGVSitzbjuZ3shNUXBrqLxEJyAmKv2".to_owned(), vec![0, 5]),
		("uY2MYFqCLpgx5XQcjdtZK286AwWV7IBGEfuS9yTmbJvkzoUPeYRHr4iDs3naN0".to_owned(), vec![0, 6]),
		("74dID7X28VLQhBlnGmjZrec5wTA1fqpWtK4YkaoEIM9SRNiC3gUJH0OFvsPDdy".to_owned(), vec![0, 7]),
		("30WXpesPhgKiEI5RHTY7xbB1GnytJvXOl2p0AcUjdF6waZDo9Qk8VLzMuWrqCS".to_owned(), vec![0, 8]),
		("moxr3HqLAK0GsTND6jowfZz3SUx7cQ8aC54Pl1RbIvFXmEJuBMYVeW9yrdOtin".to_owned(), vec![0, 9]),
	];

	for (id, numbers) in ids {
		assert_eq!(sqids.encode(&numbers).unwrap(), id);
		assert_eq!(sqids.decode(&id), numbers);
	}
}

#[test]
fn min_lengths() {
	for &min_length in &[0, 1, 5, 10, Options::default().alphabet.len() as u8] {
		for numbers in &[
			vec![u64::MIN],
			vec![0, 0, 0, 0, 0],
			vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
			vec![100, 200, 300],
			vec![1_000, 2_000, 3_000],
			vec![1_000_000],
			vec![u64::MAX],
		] {
			let sqids = Sqids::new(Some(Options::new(None, Some(min_length), None))).unwrap();

			let id = sqids.encode(&numbers).unwrap();
			assert!(id.len() >= min_length as usize);
			assert_eq!(sqids.decode(&id), *numbers);
		}
	}
}

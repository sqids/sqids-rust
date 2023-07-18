use sqids::*;

#[test]
fn simple() {
	let sqids = Sqids::new(Some(Options::new(None, Some(Options::default().alphabet.len()), None)))
		.unwrap();

	let numbers = vec![1, 2, 3];
	let id = "75JILToVsGerOADWmHlY38xvbaNZKQ9wdFS0B6kcMEtnRpgizhjU42qT1cd0dL".to_owned();

	assert_eq!(sqids.encode(&numbers).unwrap(), id);
	assert_eq!(sqids.decode(&id), numbers);
}

#[test]
fn incremental_numbers() {
	let sqids = Sqids::new(Some(Options::new(None, Some(Options::default().alphabet.len()), None)))
		.unwrap();

	let ids = vec![
		("jf26PLNeO5WbJDUV7FmMtlGXps3CoqkHnZ8cYd19yIiTAQuvKSExzhrRghBlwf".to_owned(), vec![0, 0]),
		("vQLUq7zWXC6k9cNOtgJ2ZK8rbxuipBFAS10yTdYeRa3ojHwGnmMV4PDhESI2jL".to_owned(), vec![0, 1]),
		("YhcpVK3COXbifmnZoLuxWgBQwtjsSaDGAdr0ReTHM16yI9vU8JNzlFq5Eu2oPp".to_owned(), vec![0, 2]),
		("OTkn9daFgDZX6LbmfxI83RSKetJu0APihlsrYoz5pvQw7GyWHEUcN2jBqd4kJ9".to_owned(), vec![0, 3]),
		("h2cV5eLNYj1x4ToZpfM90UlgHBOKikQFvnW36AC8zrmuJ7XdRytIGPawqYEbBe".to_owned(), vec![0, 4]),
		("7Mf0HeUNkpsZOTvmcj836P9EWKaACBubInFJtwXR2DSzgYGhQV5i4lLxoT1qdU".to_owned(), vec![0, 5]),
		("APVSD1ZIY4WGBK75xktMfTev8qsCJw6oyH2j3OnLcXRlhziUmpbuNEar05QCsI".to_owned(), vec![0, 6]),
		("P0LUhnlT76rsWSofOeyRGQZv1cC5qu3dtaJYNEXwk8Vpx92bKiHIz4MgmiDOF7".to_owned(), vec![0, 7]),
		("xAhypZMXYIGCL4uW0te6lsFHaPc3SiD1TBgw5O7bvodzjqUn89JQRfk2Nvm4JI".to_owned(), vec![0, 8]),
		("94dRPIZ6irlXWvTbKywFuAhBoECQOVMjDJp53s2xeqaSzHY8nc17tmkLGwfGNl".to_owned(), vec![0, 9]),
	];

	for (id, numbers) in ids {
		assert_eq!(sqids.encode(&numbers).unwrap(), id);
		assert_eq!(sqids.decode(&id), numbers);
	}
}

#[test]
fn min_lengths() {
	for &min_length in &[0, 1, 5, 10, Options::default().alphabet.len()] {
		for numbers in &[
			vec![Sqids::min_value()],
			vec![0, 0, 0, 0, 0],
			vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
			vec![100, 200, 300],
			vec![1_000, 2_000, 3_000],
			vec![1_000_000],
			vec![Sqids::max_value()],
		] {
			let sqids = Sqids::new(Some(Options::new(None, Some(min_length), None))).unwrap();

			let id = sqids.encode(&numbers).unwrap();
			assert!(id.len() >= min_length);
			assert_eq!(sqids.decode(&id), *numbers);
		}
	}
}

#[test]
fn out_of_range_invalid_min_length() {
	assert_eq!(
		Sqids::new(Some(Options::new(None, Some(Options::default().alphabet.len() + 1), None)))
			.err()
			.unwrap(),
		Error::MinLength { min: 0, max: Options::default().alphabet.len() }
	);
}

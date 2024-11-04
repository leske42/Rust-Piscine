	#[test]
	#[should_panic]
	fn test_both_empty() {
		big_add(b"", b"");
	}

	#[test]
	#[should_panic]
	fn test_a_empty() {
		big_add(b"", b"42");
	}

	#[test]
	#[should_panic]
	fn test_b_empty() {
		big_add(b"42", b"");
	}

	#[test]
	fn test_invalid_bytes() {
		let input_strings = vec![b"/42", b"42/", b":42", b"42:",  b" 42", b"42 ", b"+42", b"-42", &[52, 50, 0]];
		for a in &input_strings {
			for b in &input_strings {
				let result = std::panic::catch_unwind(|| {
					big_add(*a, *b);
				});
				match result {
					Ok(_) => panic!("Invalid input not correctly handled"),
					Err(payload) => {
						let payload_str = match payload.downcast_ref::<String>() {
							Some(s) => s.as_str(),
							None => match payload.downcast_ref::<&str>() {
								Some(s) => s,
								None => continue
							}
						};
						if payload_str.contains("subtract with overflow") {
							panic!("Invalid input not correctly handled");
						}
					}
				}
			}
		}
	}

	#[test]
	fn test_0() {
		assert_eq!(big_add(b"1", b"2"), b"3");
	}

	#[test]
	fn test_1() {
		assert_eq!(big_add(b"8", b"4"), b"12");
	}

	#[test]
	fn test_2() {
		assert_eq!(big_add(b"0", b"0"), b"0");
	}

	#[test]
	fn test_3() {
		assert_eq!(big_add(b"00003674", b"1757"), b"5431");
	}

	#[test]
	fn test_4() {
		assert_eq!(big_add(b"3319", b"00001259"), b"4578");
	}

	#[test]
	fn test_5() {
		assert_eq!(big_add(b"42", b"57"), b"99");
	}

	#[test]
	fn test_6() {
		assert_eq!(big_add(b"42", b"58"), b"100");
	}

	#[test]
	fn test_7() {
		assert_eq!(big_add(b"42", b"59"), b"101");
	}

	#[test]
	fn test_8() {
		assert_eq!(big_add(b"50000", b"50000"), b"100000");
	}

	#[test]
	fn test_9() {
		assert_eq!(big_add(b"340282366920938463463374607431768211456", b"18446744073709551616"), b"340282366920938463481821351505477763072");
	}

	#[test]
	fn test_10() {
		assert_eq!(big_add(b"000000", b"00000"), b"0");
	}


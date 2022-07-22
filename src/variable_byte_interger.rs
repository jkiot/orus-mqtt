use std::error::Error;

const VAR_BYTE_MAX_NUM_OF_BYTES: usize = 4;
const VAR_BYTE_MIN_VALUE: u32 = 0;
const VAR_BYTE_MAX_VALUE: u32 = 268_435_455;
const VAR_BYTE_FACTOR_VALUE_127: u8 = 127;
const VAR_BYTE_FACTOR_VALUE_128: u8 = 128;
const VAR_BYTE_MAX_VALUE_MAX_MULTIPLIER: u32 = 128 * 128 * 128;

pub fn encode(mut value: u32) -> Result<Vec<u8>, String> {
    if value < VAR_BYTE_MIN_VALUE || value > VAR_BYTE_MAX_VALUE {
        // TODO: This error shall be set from proper enum with error codes
        return Err(String::from("Malformed Packet"));
    }

    let mut encoded_vec: Vec<u8> = Vec::new();

    loop {
        let mut encoded_byte = (value % VAR_BYTE_FACTOR_VALUE_128 as u32) as u8;
        value /= VAR_BYTE_FACTOR_VALUE_128 as u32;
        // if there are more data to encode, set the top bit of this byte
        if value > 0 {
            encoded_byte |= VAR_BYTE_FACTOR_VALUE_128;
            encoded_vec.push(encoded_byte);
        } else {
            encoded_vec.push(encoded_byte);
            break;
        }
    }

    if encoded_vec.len() > VAR_BYTE_MAX_NUM_OF_BYTES {
        // TODO: This error shall be set from proper enum with error codes
        return Err(String::from("Malformed Packet"));
    }

    Ok(encoded_vec)
}

pub fn decode(encoded_vec: &[u8]) -> Result<u32, String> {
    if encoded_vec.len() > VAR_BYTE_MAX_NUM_OF_BYTES || !is_byte_sequence_valid(encoded_vec) {
        // TODO: This error shall be set from proper enum with error codes
        return Err(String::from("Malformed Packet"));
    }

    let mut mutiplier: u32 = 1;
    let mut value: u32 = 0;

    for encoded_byte in encoded_vec {
        value += (encoded_byte & VAR_BYTE_FACTOR_VALUE_127) as u32 * mutiplier;

        if mutiplier > VAR_BYTE_MAX_VALUE_MAX_MULTIPLIER {
            // TODO: This error shall be set from proper enum with error codes
            return Err(String::from("Malformed Packet"));
        }

        mutiplier *= 128;
    }
    Ok(value)
}

pub fn is_byte_sequence_valid(bytes_seq: &[u8]) -> bool {
    for (i, value) in bytes_seq.iter().enumerate() {
        if (i + 1) == bytes_seq.len() {
            if (value & VAR_BYTE_FACTOR_VALUE_128) != 0 {
                return false;
            }
        } else if value & VAR_BYTE_FACTOR_VALUE_128 == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_CASE_STR: &str = "Test Case:";

    #[test]
    fn encode_parser_when_values_are_valid() {
        /*
        test_data: is a vector of tuples with following fields:
            String: Test case name
            u32: value to be encoded
            Vec<u8>: expected vector with bytes after the value is encoded
        */
        let mut test_data: Vec<(String, u32, Vec<u8>)> = Vec::new();
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            0,
            [0x00].to_vec(),
        ));
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            127,
            [0x7F].to_vec(),
        ));
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            128,
            [0x80, 0x01].to_vec(),
        ));
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            16_383,
            [0xFF, 0x7F].to_vec(),
        ));
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            16_384,
            [0x80, 0x80, 0x01].to_vec(),
        ));
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            2_097_151,
            [0xFF, 0xFF, 0x7F].to_vec(),
        ));
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            2_097_152,
            [0x80, 0x80, 0x80, 0x01].to_vec(),
        ));
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            268_435_455,
            [0xFF, 0xFF, 0xFF, 0x7F].to_vec(),
        ));

        for (test_case, value, expected_vec) in test_data {
            println!("Running: {}", test_case);

            let encoded_res = encode(value);

            match encoded_res {
                Ok(encoded_vec) => {
                    assert_eq!(
                        expected_vec.len(),
                        encoded_vec.len(),
                        "Arrays don't have the same length"
                    );
                    assert!(
                        expected_vec
                            .iter()
                            .zip(encoded_vec.iter())
                            .all(|(a, b)| a == b),
                        "Arrays are not equal"
                    );
                }
                Err(_) => panic!("This test shall always return Ok(Vec<u8>)"),
            }
        }
    }

    #[test]
    fn encode_parser_when_values_are_out_of_range() {
        /*
        test_data: is a vector of tuples with following fields:
            String: Test case name
            u32: value to be encoded
            String: expected string in Err() returned by the encode function
        */
        let mut test_data: Vec<(String, u32, &str)> = Vec::new();

        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            268_435_456,
            "Malformed Packet",
        ));
        test_data.push((
            String::from(format!("{}{}", TEST_CASE_STR, test_data.len())),
            268_435_457,
            "Malformed Packet",
        ));

        for (test_case, value, expected_err) in test_data {
            println!("Running: {}", test_case);

            let encoded_res = encode(value);

            match encoded_res {
                Err(err) => assert_eq!(expected_err.to_string(), err),
                Ok(_) => panic!("This test shall always return Err()"),
            }
        }
    }

    #[test]
    fn decode_parser_when_input_vector_elements_are_valid() {
        /*
        test_data: is a vector of tuples with following fields:
            String : Test case name
            Vec<u8> : vector that holds encoded bytes to be decoded
            u32 : expected value after decoding
        */
        let mut test_data: Vec<(String, Vec<u8>, u32)> = Vec::new();
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            [0x00].to_vec(),
            0,
        ));
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            [0x7F].to_vec(),
            127,
        ));
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            [0x80, 0x01].to_vec(),
            128,
        ));
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            [0xFF, 0x7F].to_vec(),
            16_383,
        ));
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            [0x80, 0x80, 0x01].to_vec(),
            16_384,
        ));
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            [0xFF, 0xFF, 0x7F].to_vec(),
            2_097_151,
        ));
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            [0x80, 0x80, 0x80, 0x01].to_vec(),
            2_097_152,
        ));
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            [0xFF, 0xFF, 0xFF, 0x7F].to_vec(),
            268_435_455,
        ));

        for (test_case, encoded_vec, expected_value) in test_data {
            println!("Running: {}", test_case);

            let decoded_res = decode(&encoded_vec);

            match decoded_res {
                Ok(decoded_value) => {
                    assert_eq!(expected_value, decoded_value, "Erro to decode value.");
                }
                Err(_) => panic!("This test shall always return Ok(Vec<u8>)."),
            }
        }
    }

    #[test]
    fn decode_parser_when_input_vector_has_more_elements_than_allowed() {
        /*
        test_data: is a vector of tuples with following fields:
            String : Test case name
            Vec<u8> : vector that holds encoded bytes to be decoded
            &str : expected value after decoding
        */
        let mut test_data: Vec<(String, Vec<u8>, &str)> = Vec::new();

        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            [0x80, 0x80, 0x80, 0x80, 0x01].to_vec(),
            "Malformed Packet",
        ));

        for (test_case, encoded_vec, expected_value) in test_data {
            println!("Running: {}", test_case);

            let encoded_res = decode(&encoded_vec);

            match encoded_res {
                Err(err) => assert_eq!(expected_value.to_string(), err),
                Ok(_) => panic!("This test shall always return Err()"),
            }
        }
    }

    #[test]
    fn decode_parser_when_input_vector_has_invalid_sequence_of_bytes() {
        /*
        test_data: is a vector of tuples with following fields:
            String : Test case name
            Vec<u8> : vector that holds encoded bytes to be decoded
            &str : expected value after decoding

        NOTE: the aim of this test is to check that the decode function is
        robust against wrong sequence of bytes. For example:
        0x80, 0x80, 0x80, 0x01 = is a valid sequence
        0x00, 0x01, 0x80 = is not value as the 2nd byte indicates  (0x01 AND 128 == 0)
        there should not be following bytes after it but there is a 3th byte v0x80
        */
        let mut test_data: Vec<(String, Vec<u8>, &str)> = Vec::new();

        // 1st is invalid as is states there is more bytes after it but there are not.
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            [0x80].to_vec(),
            "Malformed Packet",
        ));

        // bytes after 1st one are invalid
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            [0x00, 0x01].to_vec(),
            "Malformed Packet",
        ));

        // bytes after 2nd one are invalid
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            [0x80, 0x01, 0x01].to_vec(),
            "Malformed Packet",
        ));

        // bytes after 3th one are invalid
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            [0x80, 0x80, 0x10, 0x01].to_vec(),
            "Malformed Packet",
        ));

        // 4th is invalid as is states there is more bytes after it but there are not.
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            [0x80, 0x80, 0x80, 0x80].to_vec(),
            "Malformed Packet",
        ));

        for (test_case, encoded_vec, expected_value) in test_data {
            println!("Running: {}", test_case);

            let encoded_res = decode(&encoded_vec);

            match encoded_res {
                Err(err) => assert_eq!(expected_value.to_string(), err),
                Ok(_) => panic!("This test shall always return Err()"),
            }
        }
    }

    #[test]
    fn is_byte_sequence_valid_when_values_are_not_valid() {
        /*
        test_data: is a vector of tuples with following fields:
            String : Test case name
            Vec<u8> : vector that holds encoded bytes to be decoded
            bool : expected value after decoding

        NOTE: the aim of this test is to check that the decode function is
        robust against wrong sequence of bytes. For example:
        0x80, 0x80, 0x80, 0x01 = is a valid sequence
        0x00, 0x01, 0x80 = is not value as the 2nd byte indicates  (0x01 AND 128 == 0)
        there should not be following bytes after it but there is a 3th byte v0x80
        */
        let mut test_data: Vec<(String, Vec<u8>, bool)> = Vec::new();

        // 1st is invalid as is states there is more bytes after it but there are not.
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            [0x80].to_vec(),
            false,
        ));

        // bytes after 1st one are invalid
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            [0x00, 0x01].to_vec(),
            false,
        ));

        // bytes after 2nd one are invalid
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            [0x80, 0x01, 0x01].to_vec(),
            false,
        ));

        // bytes after 3th one are invalid
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            [0x80, 0x80, 0x10, 0x01].to_vec(),
            false,
        ));

        // 4th is invalid as is states there is more bytes after it but there are not.
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            [0x80, 0x80, 0x80, 0x80].to_vec(),
            false,
        ));

        for (test_case, encoded_vec, expected_value) in test_data {
            println!("Running: {}", test_case);

            assert_eq!(expected_value, is_byte_sequence_valid(&encoded_vec));
        }
    }

    #[test]
    fn is_byte_sequence_valid_when_values_are_valid() {
        /*
        test_data: is a vector of tuples with following fields:
            String : Test case name
            Vec<u8> : vector that holds encoded bytes to be decoded
            &str : expected value after decoding

        NOTE: the aim of this test is to check that the decode function is
        robust against wrong sequence of bytes. For example:
        0x80, 0x80, 0x80, 0x01 = is a valid sequence
        0x00, 0x01, 0x80 = is not value as the 2nd byte indicates  (0x01 AND 128 == 0)
        there should not be following bytes after it but there is a 3th byte v0x80
        */
        let mut test_data: Vec<(String, Vec<u8>, bool)> = Vec::new();

        // bytes after 1st one are invalid
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            [0x00].to_vec(),
            true,
        ));
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            [0x7F].to_vec(),
            true,
        ));
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            [0x80, 0x01].to_vec(),
            true,
        ));
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            [0xFF, 0x7F].to_vec(),
            true,
        ));
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            [0x80, 0x80, 0x01].to_vec(),
            true,
        ));
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            [0xFF, 0xFF, 0x7F].to_vec(),
            true,
        ));
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            [0x80, 0x80, 0x80, 0x01].to_vec(),
            true,
        ));
        test_data.push((
            format!("{}{}", TEST_CASE_STR, test_data.len()),
            [0xFF, 0xFF, 0xFF, 0x7F].to_vec(),
            true,
        ));

        for (test_case, encoded_vec, expected_value) in test_data {
            println!("Running: {}", test_case);

            assert_eq!(expected_value, is_byte_sequence_valid(&encoded_vec));
        }
    }
}

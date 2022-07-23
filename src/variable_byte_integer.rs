const ERROR_CODE: &'static str = "Malformed Packet";

const VAR_BYTE_MAX_NUM_OF_BYTES: usize = 4;
const VAR_BYTE_MIN_VALUE: u32 = 0;
const VAR_BYTE_MAX_VALUE: u32 = 268_435_455;
const VAR_BYTE_FACTOR_VALUE_127: u8 = 127;
const VAR_BYTE_FACTOR_VALUE_128: u8 = 128;
const VAR_BYTE_MAX_VALUE_MAX_MULTIPLIER: u32 = 128 * 128 * 128;

pub fn encode(mut value: u32) -> Result<Vec<u8>, &'static str> {
    if value < VAR_BYTE_MIN_VALUE || value > VAR_BYTE_MAX_VALUE {
        // TODO: This error shall be set from proper enum with error codes
        return Err(ERROR_CODE);
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
        Err(ERROR_CODE)
    } else {
        Ok(encoded_vec)
    }
}

pub fn decode(encoded_vec: &[u8]) -> Result<u32, &'static str> {
    if encoded_vec.is_empty()
        || encoded_vec.len() > VAR_BYTE_MAX_NUM_OF_BYTES
        || !is_byte_sequence_valid(encoded_vec)
    {
        // TODO: This error shall be set from proper enum with error codes
        return Err(ERROR_CODE);
    }

    let mut mutiplier: u32 = 1;
    let mut value: u32 = 0;

    for encoded_byte in encoded_vec {
        value += (encoded_byte & VAR_BYTE_FACTOR_VALUE_127) as u32 * mutiplier;

        if mutiplier > VAR_BYTE_MAX_VALUE_MAX_MULTIPLIER {
            // TODO: This error shall be set from proper enum with error codes
            return Err(ERROR_CODE);
        }

        mutiplier *= 128;
    }
    Ok(value)
}

pub fn is_byte_sequence_valid(bytes_seq: &[u8]) -> bool {
    if bytes_seq.is_empty() {
        return false;
    }
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

    #[test]
    fn test_encode_parser() {
        let mut test_data = Vec::new();
        test_data.push((0, Result::Ok(vec![0x00])));
        test_data.push((127, Result::Ok(vec![0x7F])));
        test_data.push((128, Result::Ok(vec![0x80, 0x01])));
        test_data.push((16_383, Result::Ok(vec![0xFF, 0x7F])));
        test_data.push((16_384, Result::Ok(vec![0x80, 0x80, 0x01])));
        test_data.push((2_097_151, Result::Ok(vec![0xFF, 0xFF, 0x7F])));
        test_data.push((2_097_152, Result::Ok(vec![0x80, 0x80, 0x80, 0x01])));
        test_data.push((268_435_455, Result::Ok(vec![0xFF, 0xFF, 0xFF, 0x7F])));
        //Values out of range
        test_data.push((268_435_456, Result::Err(ERROR_CODE)));
        test_data.push((268_435_457, Result::Err(ERROR_CODE)));

        test_data
            .into_iter()
            .enumerate()
            .for_each(|(test_case, (value, expected_result))| {
                println!("Running Test Case: {}", test_case);

                let encoded_res = encode(value);

                assert_eq!(expected_result, encoded_res);

                match encoded_res {
                    Ok(encoded_vec) => {
                        let expected_vec = expected_result.unwrap();
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
                    Err(err) => assert_eq!(expected_result.unwrap_err(), err),
                }
            });
    }

    #[test]
    fn test_decode_parser() {
        let mut test_data: Vec<(Vec<u8>, Result<u32, &str>)> = Vec::new();
        test_data.push((vec![0x00], Result::Ok(0)));
        test_data.push((vec![0x7F], Result::Ok(127)));
        test_data.push((vec![0x80, 0x01], Result::Ok(128)));
        test_data.push((vec![0xFF, 0x7F], Result::Ok(16_383)));
        test_data.push((vec![0x80, 0x80, 0x01], Result::Ok(16_384)));
        test_data.push((vec![0xFF, 0xFF, 0x7F], Result::Ok(2_097_151)));
        test_data.push((vec![0x80, 0x80, 0x80, 0x01], Result::Ok(2_097_152)));
        test_data.push((vec![0xFF, 0xFF, 0xFF, 0x7F], Result::Ok(268_435_455)));

        //input_vector_has_more_elements_than_allowed
        test_data.push((vec![0x80, 0x80, 0x80, 0x80, 0x01], Result::Err(ERROR_CODE)));
        //input_vector_is_empty
        test_data.push((vec![], Result::Err(ERROR_CODE)));
        /*
        NOTE: the aim of inputs below are to check that the decode function is
        robust against wrong sequence of bytes. For example:
        0x80, 0x80, 0x80, 0x01 = is a valid sequence
        0x00, 0x01, 0x80 = is not value as the 2nd byte indicates  (0x01 AND 128 == 0)
        there should not be following bytes after it but there is a 3th byte v0x80
        */
        // 1st is invalid as is states there is more bytes after it but there are not.
        test_data.push((vec![0x80], Result::Err(ERROR_CODE)));
        // bytes after 1st one are invalid
        test_data.push((vec![0x00, 0x01], Result::Err(ERROR_CODE)));
        // bytes after 2nd one are invalid
        test_data.push((vec![0x80, 0x01, 0x01], Result::Err(ERROR_CODE)));
        // bytes after 3th one are invalid
        test_data.push((vec![0x80, 0x80, 0x10, 0x01], Result::Err(ERROR_CODE)));
        // 4th is invalid as is states there is more bytes after it but there are not.
        test_data.push((vec![0x80, 0x80, 0x80, 0x80], Result::Err(ERROR_CODE)));

        test_data
            .into_iter()
            .enumerate()
            .for_each(|(test_case, (value, expected_result))| {
                println!("Running Test Case: {}", test_case);

                let decoded_result = decode(&value);

                assert_eq!(expected_result, decoded_result);

                match decoded_result {
                    Ok(res) => assert_eq!(expected_result.unwrap(), res, "Erro to decode value."),
                    Err(err) => assert_eq!(expected_result.unwrap_err(), err),
                }
            });
    }

    #[test]
    fn test_is_byte_sequence_valid() {
        let mut test_data = Vec::new();
        test_data.push((vec![0x00], true));
        test_data.push((vec![0x7F], true));
        test_data.push((vec![0x80, 0x01], true));
        test_data.push((vec![0xFF, 0x7F], true));
        test_data.push((vec![0x80, 0x80, 0x01], true));
        test_data.push((vec![0xFF, 0xFF, 0x7F], true));
        test_data.push((vec![0x80, 0x80, 0x80, 0x01], true));
        test_data.push((vec![0xFF, 0xFF, 0xFF, 0x7F], true));
        /*
        NOTE: the aim of inputs below are to check that the decode function is
        robust against wrong sequence of bytes. For example:
        0x80, 0x80, 0x80, 0x01 = is a valid sequence
        0x00, 0x01, 0x80 = is not value as the 2nd byte indicates  (0x01 AND 128 == 0)
        there should not be following bytes after it but there is a 3th byte v0x80
        */
        // empty vector
        test_data.push((vec![], false));
        // 1st is invalid as is states there is more bytes after it but there are not.
        test_data.push((vec![0x80], false));
        // bytes after 1st one are invalid
        test_data.push((vec![0x00, 0x01], false));
        // bytes after 2nd one are invalid
        test_data.push((vec![0x80, 0x01, 0x01], false));
        // bytes after 3th one are invalid
        test_data.push((vec![0x80, 0x80, 0x10, 0x01], false));
        // 4th is invalid as is states there is more bytes after it but there are not.
        test_data.push((vec![0x80, 0x80, 0x80, 0x80], false));

        test_data
            .into_iter()
            .enumerate()
            .for_each(|(test_case, (value, expected_result))| {
                println!("Running Test Case: {}", test_case);
                assert_eq!(expected_result.to_owned(), is_byte_sequence_valid(&value));
            });
    }
}

pub const HEX: &str = "0123456789abcdef";
pub const BASE64: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Convert a single hexadecimal character into the appropriate numeric value:
/// b'0' => 0, b'1' => 1, ..., b'f' => 15
pub fn hex2byte(byte: u8) -> Result<u8, &'static str> {
    if b'0' <= byte && byte <= b'9' {
        return Ok(byte - b'0');
    } else if b'a' <= byte && byte <= b'f' {
        return Ok(byte - b'a' + 10);
    }
    return Err("Invalid hexadecimal character");
}

/// Convert a single base-64 character into the appropriate numeric value:
/// b'A' => 0, b'B' => 1, ..., b'/' => 63
pub fn base642byte(byte: u8) -> Result<u8, &'static str> {
    if b'A' <= byte && byte <= b'Z' {
        return Ok(byte - b'A');
    } else if b'a' <= byte && byte <= b'z' {
        return Ok(byte - b'a' + 26);
    } else if b'0' <= byte && byte <= b'9' {
        return Ok(byte - b'0' + 52);
    } else if byte == b'+' {
        return Ok(62);
    } else if byte == b'/' {
        return Ok(63);
    }
    return Err("Invalid base-64 character");
}

/// Given a hexadecimal string, return a vector of bytes that the input string
/// encodes
pub fn hex2bytes(hex: &str) -> Result<Vec<u8>, &'static str> {
    if hex.len() % 2 != 0 {
        return Err("Incomplete byte!");
    }
    let mut bytes: Vec<u8> = Vec::new();
    let mut byte: u8 = 0;

    for (i, &c) in hex.as_bytes().iter().enumerate() {
        if i % 2 == 0 {
            // the first 4 bits
            byte = hex2byte(c).unwrap() * 16;
        } else {
            // the last 4 bits
            byte += hex2byte(c).unwrap();
            bytes.push(byte);
        }
    }
    return Ok(bytes);
}

/// Given an array of bytes, return the base-64 encoded string with padding
pub fn bytes2base64(bytes: &[u8]) -> String {
    let base64str: String = String::new();
    let mut padded: Vec<u8> = Vec::new();
    for &byte in bytes { padded.push(byte); }

    // b'=' is 61 => 111101
    match bytes.len() % 3 {
        1 => {
            // padd 00001111 01111101 (4 zeros, two b'=')
            padded.push(0b00001111);
            padded.push(0b01111101);
        },
        2 => {
            // pad 00111101 (1 zeros, 1 b'=')
            padded.push(0b00111101);
        },
        _ => (),
    }

    let triplebytes: Vec<TripleByte> = TripleByte::from_bytes(&padded);
    for triplebyte in triplebytes {
        for byte in triplebyte.to_base64() {
            base64str.push(byte);
        }
    }

    return base64str;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex2byte() {
        for (i, &byte) in HEX.as_bytes().iter().enumerate() {
            assert_eq!(i, hex2byte(byte).unwrap() as usize);
        }
    }

    #[test]
    fn test_base642byte() {
        for (i, &byte) in BASE64.as_bytes().iter().enumerate() {
            assert_eq!(i, base642byte(byte).unwrap() as usize);
        }
    }

    #[test]
    fn test_hex2bytes() {
        assert_eq!(hex2bytes("00").unwrap(), vec![0]);
        assert_eq!(hex2bytes("0f").unwrap(), vec![15]);
        assert_eq!(hex2bytes("10").unwrap(), vec![16]);
        assert_eq!(hex2bytes("ff").unwrap(), vec![255]);
    }

    #[test]
    fn convert() {
        let input_hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected_b64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        let bytearr = hex2bytes(input_hex).unwrap();
        let result_b64 = bytes2base64(&bytearr);
        assert_eq!(&result_b64, expected_b64);
    }
}

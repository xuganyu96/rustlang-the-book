/**
 * CryptoPals Challenge 1: convert hex to base64
 * ---------------------------------------------
 * Some context on base64 encoding: A-Z, a-z, 0-9, +, /
 * the padding character is "=".
 * Encode 24 bits at a time. If the remainder block is exactly 8 bits, pad
 * with 4 bits of 0 and 2 padding character. If the remainder block is
 * exactly 16 bits, pad with 2 bits of 0 and 2 padding character.
 */

/// Given a "char" (u8), return the hexadecimal value that is encodes:
fn base16lookup(byte: u8) -> Result<u8, &'static str> {
    match byte {
        b'0' => return Ok(0),
        b'1' => return Ok(1),
        b'2' => return Ok(2),
        b'3' => return Ok(3),
        b'4' => return Ok(4),
        b'5' => return Ok(5),
        b'6' => return Ok(6),
        b'7' => return Ok(7),
        b'8' => return Ok(8),
        b'9' => return Ok(9),
        b'a' => return Ok(10),
        b'b' => return Ok(11),
        b'c' => return Ok(12),
        b'd' => return Ok(13),
        b'e' => return Ok(14),
        b'f' => return Ok(15),
        _ => return Err("Invalid hex character"),
    }
}

/// Given a string that contains an even number of characters 0-9a-f, return
/// the byte array that the input hex string encodes. Each hex character
/// encodes 4 bits, so the conversion translates two hex character into a
/// single byte
pub fn base16decode(hexstr: &str) -> Result<Vec<u8>, &'static str> {
    if hexstr.len() % 2 != 0 { return Err("Odd number of characters"); }

    let mut bytes: Vec<u8> = Vec::new();
    let mut byte: u8 = 0;

    for (i, &c) in hexstr.as_bytes().iter().enumerate() {
        match i % 2 {
            0 => {
                byte = base16lookup(c)?;
                byte = byte << 4;
            },
            _ => {
                byte += base16lookup(c)?;
                bytes.push(byte);
            },
        }
    }
    
    return Ok(bytes);
}

/// Given a numerical value between 0 and 63, return the byte that encodes
/// this numeric value in base 64
fn base64lookup(val: u8) -> char {
    match val {
        0 => 'A',
        1 => 'B',
        2 => 'C',
        3 => 'D',
        4 => 'E',
        5 => 'F',
        6 => 'G',
        7 => 'H',
        8 => 'I',
        9 => 'J',
        10 => 'K',
        11 => 'L',
        12 => 'M',
        13 => 'N',
        14 => 'O',
        15 => 'P',
        16 => 'Q',
        17 => 'R',
        18 => 'S',
        19 => 'T',
        20 => 'U',
        21 => 'V',
        22 => 'W',
        23 => 'X',
        24 => 'Y',
        25 => 'Z',
        26 => 'a',
        27 => 'b',
        28 => 'c',
        29 => 'd',
        30 => 'e',
        31 => 'f',
        32 => 'g',
        33 => 'h',
        34 => 'i',
        35 => 'j',
        36 => 'k',
        37 => 'l',
        38 => 'm',
        39 => 'n',
        40 => 'o',
        41 => 'p',
        42 => 'q',
        43 => 'r',
        44 => 's',
        45 => 't',
        46 => 'u',
        47 => 'v',
        48 => 'w',
        49 => 'x',
        50 => 'y',
        51 => 'z',
        52 => '0',
        53 => '1',
        54 => '2',
        55 => '3',
        56 => '4',
        57 => '5',
        58 => '6',
        59 => '7',
        60 => '8',
        61 => '9',
        62 => '+',
        63 => '/',
        _ => '=',
    }
}

#[derive(Debug)]
pub struct Base64Block(char, char, char, char);

impl Base64Block {
    pub fn from_three_bytes(b1: u8, b2: u8, b3: u8) -> Self {
        let val: u32 = (b3 as u32) + ((b2 as u32) << 8) + ((b1 as u32) << 16);

        return Base64Block{
            0: base64lookup(((val >> 18) % 64) as u8),
            1: base64lookup(((val >> 12) % 64) as u8),
            2: base64lookup(((val >> 6) % 64) as u8),
            3: base64lookup((val % 64) as u8),
        };
    }
}

/// Given byte array, return a String with characters in "A-Za-z0-9+/=" that
/// is the base 64 encoding of the byte array
/// My implementation of padding is to create a copy of the original byte arr
pub fn base64encode(bytes: &[u8]) -> String {
    let mut base64str = String::new();
    let mut i = 0; // index of the first byte of the block; increment by 3

    while i + 2 < bytes.len() {
        let block = Base64Block::from_three_bytes(bytes[i], bytes[i+1], bytes[i+2]);
        base64str.push(block.0);
        base64str.push(block.1);
        base64str.push(block.2);
        base64str.push(block.3);
        i += 3;
    }

    match bytes.len() % 3 {
        1 => { // pad 4 bits of 0 and 2 "="
            let block = Base64Block::from_three_bytes(bytes[bytes.len()-1], 0b00001111, 0b11111111);
            base64str.push(block.0);
            base64str.push(block.1);
            base64str.push('=');
            base64str.push('=');
        },
        2 => { // pad 2 bits of 0 and 1 "="
            let block = Base64Block::from_three_bytes(bytes[bytes.len()-2], bytes[bytes.len()-1], 0b00111111);
            base64str.push(block.0);
            base64str.push(block.1);
            base64str.push(block.2);
            base64str.push('=');
        },
        _ => (),
    }

    base64str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_decode() {
        let input = "0000";
        assert_eq!(
            base16decode(input).unwrap(),
            hex::decode(input).unwrap()
        );

        let input = "abcd";
        assert_eq!(
            base16decode(input).unwrap(),
            hex::decode(input).unwrap()
        );
    }

    #[test]
    fn convert_hex_to_base64() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let bytes = base16decode(input).unwrap();
        let output = base64encode(&bytes);

        assert_eq!(output, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    }
}


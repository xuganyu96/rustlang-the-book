/**
 * CryptoPals Challenge 1: convert hex to base64
 *
 * My approach is to
 * 1. Convert hex String to byte array Vec<u8>
 * 2. Add appropriate padding to byte array &mut [u8]
 * 3. Convert byte array to TripleByte array where each TripleByte contains
 *    three bytes
 * 4. Each TripleByte correspond to four base-64 characters
 */
pub struct TripleByte<u8, u8, u8>;
pub struct Base64Block<u8, u8, u8, u8>;

impl TripleByte {
    pub fn from_bytes(bytes: &[u8]) -> Vec<Self> {}

    pub fn base64_encode(&self) -> Base64Block {}
}

impl Base64Block {
    pub fn to_string(&self) -> String {}
}

pub fn hex2bytes(hex: &str) -> Result<Vec<u8>, &'static str> {}

pub fn base64pad(bytes: &mut [u8]) {}

pub fn convert(hex: &str) -> Result<String, &'static str> {
    let mut bytes = hex2bytes(hex)?;
    base64pad(&mut bytes);

    let triplebytes: Vec<TripleByte> = TripleByte::from_bytes(&bytes);
    let base64_blocks: Vec<Base64Block> = Vec::new();

    for triplebyte in triplebytes {
        base64_blocks.push(triplebyte.base64_encode());
    }

    let base64str = String::new();
    for block in base64_blocks {
        for byte in block.to_string().as_bytes().iter() {
            base64str.push(byte);
        }
    }
    return Ok(base64str)
}


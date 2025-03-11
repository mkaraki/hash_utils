use super::Hasher;

pub struct Adler32;

impl Hasher for Adler32 {
    fn hash_name(&self) -> &'static str {
        "Adler32"
    }
    fn active_bits(&self) -> &'static u32 {
        &32
    }
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use std::hash::Hasher;
        let mut hasher = pruefung::adler32::Adler32::default();
        hasher.write(data);
        let result: u32 = hasher.finish() as u32;
        return result.to_be_bytes().to_vec();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_properties() {
        let adler32 = Adler32;

        assert_eq!(adler32.hash_name(), "Adler32");
        assert_eq!(adler32.active_bits(), &32);
    }

    #[test]
    fn hash_validate() {
        use hex::decode;
        let adler32 = Adler32;

        assert_eq!(adler32.hash(b"a"), decode("00620062").unwrap());
        assert_eq!(adler32.hash(b"1234567890"), decode("0b2c020e").unwrap());
        assert_eq!(adler32.hash(b"12345678901234567890123456789012345678901234567890123456789012345678901234567890"), decode("97b61069").unwrap());
    }
}

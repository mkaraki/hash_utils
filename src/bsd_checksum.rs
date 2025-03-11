use super::Hasher;

pub struct BsdChecksum;

impl Hasher for BsdChecksum {
    fn hash_name(&self) -> &'static str {
        "BSD Checksum"
    }
    fn active_bits(&self) -> &'static u32 {
        &16
    }
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use std::hash::Hasher;
        let mut hasher = pruefung::bsd::Bsd::default();
        hasher.write(data);
        let result: u16 = hasher.finish() as u16;
        return result.to_be_bytes().to_vec();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_validate() {
        use hex::decode;
        let bsd_checksum = BsdChecksum;

        assert_eq!(bsd_checksum.hash(b"abc"), decode("40ac").unwrap());
        assert_eq!(bsd_checksum.hash(b"12345678901234567890123456789012345678901234567890123456789012345678901234567890"), decode("5555").unwrap());
    }
}
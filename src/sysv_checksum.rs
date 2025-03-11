use super::Hasher;

pub struct SysVChecksum;

impl Hasher for SysVChecksum {
    fn hash_name(&self) -> &'static str {
        "SysV checksum"
    }
    fn active_bits(&self) -> &'static u32 {
        &16
    }
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use std::hash::Hasher;
        let mut hasher = pruefung::sysv::SysV::default();
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
        let hasher = SysVChecksum;

        assert_eq!(hasher.hash(b"abc"), decode("0126").unwrap());
        assert_eq!(hasher.hash(b"12345678901234567890123456789012345678901234567890123456789012345678901234567890"), decode("1068").unwrap());
    }
}
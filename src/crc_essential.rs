use crc::*;
use crate::Hasher;

pub struct Crc32;
impl Hasher for Crc32 {
    fn hash_name(&self) -> &'static str {
        "CRC32 (B)"
    }
    fn active_bits(&self) -> &'static u32 {
        &32
    }
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let hasher = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        return hasher.checksum(data).to_be_bytes().to_vec();
    }
}

pub struct Crc32C;
impl Hasher for Crc32C {
    fn hash_name(&self) -> &'static str {
        "CRC32C"
    }
    fn active_bits(&self) -> &'static u32 {
        &32
    }
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let hasher = Crc::<u32>::new(&CRC_32_ISCSI);
        return hasher.checksum(data).to_be_bytes().to_vec();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc_32_hash_validate() {
        use hex::decode;
        let crc = Crc32;

        assert_eq!(crc.hash(b"a"), decode("e8b7be43").unwrap());
        assert_eq!(crc.hash(b"12345678901234567890123456789012345678901234567890123456789012345678901234567890"), decode("7ca94a72").unwrap());
    }

    #[test]
    fn crc_32_c_hash_validate() {
        use hex::decode;
        let crc = Crc32C;

        assert_eq!(crc.hash(b"abc"), decode("364b3fb7").unwrap());
        assert_eq!(crc.hash(b"12345678901234567890123456789012345678901234567890123456789012345678901234567890"), decode("477a6781").unwrap());
    }
}

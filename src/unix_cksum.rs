use crc::*;
use crate::Hasher;

pub struct UnixCksum;
impl Hasher for UnixCksum {
    fn hash_name(&self) -> &'static str {
        "Unix cksum"
    }
    fn active_bits(&self) -> &'static u32 {
        &32
    }
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let hasher = Crc::<u32>::new(&CRC_32_CKSUM);
        return hasher.checksum(data).to_be_bytes().to_vec();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_validate() {
        use hex::decode;
        let crc = UnixCksum;

        assert_eq!(crc.hash(b"abc"), decode("48aa78a2").unwrap());
        assert_eq!(crc.hash(b"12345678901234567890123456789012345678901234567890123456789012345678901234567890"), decode("73A0B3A8").unwrap());
    }
}

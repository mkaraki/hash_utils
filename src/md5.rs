use super::Hasher;

pub struct Md5;

impl Hasher for Md5 {
    fn hash_name(&self) -> &'static str {
        "md5"
    }
    fn active_bits(&self) -> &'static u32 {
        &128
    }
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        md5::compute(data).to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_validate() {
        use hex::decode;
        let hasher = Md5;

        assert_eq!(hasher.hash(b"abc"), decode("900150983cd24fb0d6963f7d28e17f72").unwrap());
        assert_eq!(hasher.hash(b"12345678901234567890123456789012345678901234567890123456789012345678901234567890"), decode("57edf4a22be3c955ac49da2e2107b67a").unwrap());
    }
}
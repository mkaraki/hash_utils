use super::Hasher;

pub struct Fnv0_32;
impl Hasher for Fnv0_32 {
    fn hash_name(&self) -> &'static str {
        "FNV-0 32 bit"
    }
    fn active_bits(&self) -> &'static u32 {
        &32
    }
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use std::hash::Hasher;
        let mut hasher = pruefung::fnv::fnv32::Fnv32z::default();
        hasher.write(data);
        let result: u32 = hasher.finish() as u32;
        return result.to_be_bytes().to_vec();
    }
}

pub struct Fnv0_64;
impl Hasher for Fnv0_64 {
    fn hash_name(&self) -> &'static str {
        "FNV-0 64 bit"
    }
    fn active_bits(&self) -> &'static u32 {
        &64
    }
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use std::hash::Hasher;
        let mut hasher = pruefung::fnv::fnv64::Fnv64z::default();
        hasher.write(data);
        let result: u64 = hasher.finish() as u64;
        return result.to_be_bytes().to_vec();
    }
}

pub struct Fnv1_32;
impl Hasher for Fnv1_32 {
    fn hash_name(&self) -> &'static str {
        "FNV-1 32 bit"
    }
    fn active_bits(&self) -> &'static u32 {
        &32
    }
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use std::hash::Hasher;
        let mut hasher = pruefung::fnv::fnv32::Fnv32::default();
        hasher.write(data);
        let result: u32 = hasher.finish() as u32;
        return result.to_be_bytes().to_vec();
    }
}

pub struct Fnv1_64;
impl Hasher for Fnv1_64 {
    fn hash_name(&self) -> &'static str {
        "FNV-1 64 bit"
    }
    fn active_bits(&self) -> &'static u32 {
        &64
    }
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use std::hash::Hasher;
        let mut hasher = pruefung::fnv::fnv64::Fnv64::default();
        hasher.write(data);
        let result: u64 = hasher.finish() as u64;
        return result.to_be_bytes().to_vec();
    }
}

pub struct Fnv1a32;
impl Hasher for Fnv1a32 {
    fn hash_name(&self) -> &'static str {
        "FNV-1a 32 bit"
    }
    fn active_bits(&self) -> &'static u32 {
        &32
    }
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use std::hash::Hasher;
        let mut hasher = pruefung::fnv::fnv32::Fnv32a::default();
        hasher.write(data);
        let result: u32 = hasher.finish() as u32;
        return result.to_be_bytes().to_vec();
    }
}

pub struct Fnv1a64;
impl Hasher for Fnv1a64 {
    fn hash_name(&self) -> &'static str {
        "FNV-1a 64 bit"
    }
    fn active_bits(&self) -> &'static u32 {
        &64
    }
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use std::hash::Hasher;
        let mut hasher = pruefung::fnv::fnv64::Fnv64a::default();
        hasher.write(data);
        let result: u64 = hasher.finish() as u64;
        return result.to_be_bytes().to_vec();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fnv_1_32_hash_validate() {
        use hex::decode;
        let hasher = Fnv1_32;

        assert_eq!(hasher.hash(b"abc"), decode("439c2f4b").unwrap());
        assert_eq!(hasher.hash(b"12345678901234567890123456789012345678901234567890123456789012345678901234567890"), decode("16121d25").unwrap());
    }

    #[test]
    fn fnv_1a_32_hash_validate() {
        use hex::decode;
        let hasher = Fnv1a32;

        assert_eq!(hasher.hash(b"abc"), decode("1a47e90b").unwrap());
        assert_eq!(hasher.hash(b"12345678901234567890123456789012345678901234567890123456789012345678901234567890"), decode("5b52cd65").unwrap());
    }

    #[test]
    fn fnv_1_64_hash_validate() {
        use hex::decode;
        let hasher = Fnv1_64;

        assert_eq!(hasher.hash(b"abc"), decode("d8dcca186bafadcb").unwrap());
        assert_eq!(hasher.hash(b"12345678901234567890123456789012345678901234567890123456789012345678901234567890"), decode("23e9297c001c2f05").unwrap());
    }

    #[test]
    fn fnv_1a_64_hash_validate() {
        use hex::decode;
        let hasher = Fnv1a64;

        assert_eq!(hasher.hash(b"abc"), decode("e71fa2190541574b").unwrap());
        assert_eq!(hasher.hash(b"12345678901234567890123456789012345678901234567890123456789012345678901234567890"), decode("95ee3578614f3045").unwrap());
    }
}

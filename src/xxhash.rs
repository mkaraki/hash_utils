use super::Hasher;

pub struct XxHash32;
impl Hasher for XxHash32 {
    fn hash_name(&self) -> &'static str {
        "XXHash 32bit"
    }

    fn active_bits(&self) -> &'static u32 {
        &32
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use std::hash::Hasher;
        let mut hasher = twox_hash::XxHash32::default();
        hasher.write(data);
        hasher.finish().to_be_bytes().to_vec()
    }
}

pub struct XxHash64;
impl Hasher for XxHash64 {
    fn hash_name(&self) -> &'static str {
        "XXHash 64bit"
    }

    fn active_bits(&self) -> &'static u32 {
        &64
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use std::hash::Hasher;
        let mut hasher = twox_hash::XxHash64::default();
        hasher.write(data);
        hasher.finish().to_be_bytes().to_vec()
    }
}

pub struct Xxh3Hash64;
impl Hasher for Xxh3Hash64 {
    fn hash_name(&self) -> &'static str {
        "XXH3 64bit"
    }

    fn active_bits(&self) -> &'static u32 {
        &64
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use std::hash::Hasher;
        let mut hasher = twox_hash::xxhash3_64::Hasher::default();
        hasher.write(data);
        hasher.finish().to_be_bytes().to_vec()
    }
}

pub struct Xxh3Hash128;
impl Hasher for Xxh3Hash128 {
    fn hash_name(&self) -> &'static str {
        "XXH3 128bit"
    }

    fn active_bits(&self) -> &'static u32 {
        &128
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let mut hasher = twox_hash::xxhash3_128::Hasher::default();
        hasher.write(data);
        hasher.finish_128().to_be_bytes().to_vec()
    }
}
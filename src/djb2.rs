use super::Hasher;

pub struct Djb2_64;
impl Hasher for Djb2_64 {
    fn hash_name(&self) -> &'static str {
        "djb2 (64bit)"
    }

    fn active_bits(&self) -> &'static u32 {
        &64
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use std::hash::Hasher;
        use hashers::oz::DJB2Hasher;
        let mut hasher = DJB2Hasher::default();
        hasher.write(data);
        hasher.finish().to_be_bytes().to_vec()
    }
}
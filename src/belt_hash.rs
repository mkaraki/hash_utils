use super::Hasher;

pub struct BeltHash;
impl Hasher for BeltHash {
    fn hash_name(&self) -> &'static str {
        "BeltHash"
    }

    fn active_bits(&self) -> &'static u32 {
        &256
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use belt_hash::Digest;
        let mut hasher = belt_hash::BeltHash::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}
use super::Hasher;

pub struct LoseLose;
impl Hasher for LoseLose {
    fn hash_name(&self) -> &'static str {
        "lose lose"
    }

    fn active_bits(&self) -> &'static u32 {
        &64
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use std::hash::Hasher;
        use hashers::oz::LoseLoseHasher;
        let mut hasher = LoseLoseHasher::default();
        hasher.write(data);
        hasher.finish().to_be_bytes().to_vec()
    }
}

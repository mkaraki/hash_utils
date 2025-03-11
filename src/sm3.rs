use super::Hasher;

pub struct Sm3;
impl Hasher for Sm3 {
    fn hash_name(&self) -> &'static str {
        "sm3"
    }
    fn active_bits(&self) -> &'static u32 {
        &256
    }
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ::sm3::Digest;
        let mut hasher = ::sm3::Sm3::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}
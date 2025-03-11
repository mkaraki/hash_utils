use super::Hasher;

pub struct Whirlpool;
impl Hasher for Whirlpool {
    fn hash_name(&self) -> &'static str {
        "Whirlpool"
    }

    fn active_bits(&self) -> &'static u32 {
        &512
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ::whirlpool::Digest;
        let mut hasher = ::whirlpool::Whirlpool::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}
use blake2::Digest;
use super::Hasher;

pub struct Blake2B512;
impl Hasher for Blake2B512 {
    fn hash_name(&self) -> &'static str {
        "Blake2b (512bit)"
    }

    fn active_bits(&self) -> &'static u32 {
        &512
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let mut hasher = blake2::Blake2b512::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct Blake2S256;
impl Hasher for Blake2S256 {
    fn hash_name(&self) -> &'static str {
        "Blake2s (256bit)"
    }

    fn active_bits(&self) -> &'static u32 {
        &256
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let mut hasher = blake2::Blake2s256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}
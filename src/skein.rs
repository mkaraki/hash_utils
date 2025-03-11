use skein::consts::{U32, U64, U128};
use super::Hasher;

pub struct Skein256;
impl Hasher for Skein256 {
    fn hash_name(&self) -> &'static str {
        "Skein-256"
    }

    fn active_bits(&self) -> &'static u32 {
        &256
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ::skein::Digest;
        let mut hasher = ::skein::Skein256::<U32>::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct Skein512;
impl Hasher for Skein512 {
    fn hash_name(&self) -> &'static str {
        "Skein-512"
    }

    fn active_bits(&self) -> &'static u32 {
        &512
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ::skein::Digest;
        let mut hasher = ::skein::Skein512::<U64>::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct Skein1024;
impl Hasher for Skein1024 {
    fn hash_name(&self) -> &'static str {
        "Skein-1024"
    }

    fn active_bits(&self) -> &'static u32 {
        &1024
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ::skein::Digest;
        let mut hasher = ::skein::Skein1024::<U128>::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}
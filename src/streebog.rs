use super::Hasher;

pub struct Streebog256;
impl Hasher for Streebog256 {
    fn hash_name(&self) -> &'static str {
        "Streebog256"
    }

    fn active_bits(&self) -> &'static u32 {
        &256
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ::streebog::Digest;
        let mut hasher = ::streebog::Streebog256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct Streebog512;
impl Hasher for Streebog512 {
    fn hash_name(&self) -> &'static str {
        "Streebog512"
    }

    fn active_bits(&self) -> &'static u32 {
        &512
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ::streebog::Digest;
        let mut hasher = ::streebog::Streebog512::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}
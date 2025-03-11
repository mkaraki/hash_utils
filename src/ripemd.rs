use super::Hasher;

pub struct Ripemd128;
impl Hasher for Ripemd128 {
    fn hash_name(&self) -> &'static str {
        "RIPEMD-128"
    }

    fn active_bits(&self) -> &'static u32 {
        &128
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ::ripemd::Digest;
        let mut hasher = ::ripemd::Ripemd128::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct Ripemd160;
impl Hasher for Ripemd160 {
    fn hash_name(&self) -> &'static str {
        "RIPEMD-160"
    }

    fn active_bits(&self) -> &'static u32 {
        &160
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ::ripemd::Digest;
        let mut hasher = ::ripemd::Ripemd160::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct Ripemd256;
impl Hasher for Ripemd256 {
    fn hash_name(&self) -> &'static str {
        "RIPEMD-256"
    }

    fn active_bits(&self) -> &'static u32 {
        &256
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ::ripemd::Digest;
        let mut hasher = ::ripemd::Ripemd256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct Ripemd320;
impl Hasher for Ripemd320 {
    fn hash_name(&self) -> &'static str {
        "RIPEMD-320"
    }

    fn active_bits(&self) -> &'static u32 {
        &320
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ::ripemd::Digest;
        let mut hasher = ::ripemd::Ripemd320::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}
use super::Hasher;

pub struct JH224;
impl Hasher for JH224 {
    fn hash_name(&self) -> &'static str {
        "JH-224"
    }

    fn active_bits(&self) -> &'static u32 {
        &224
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ::jh::Digest;
        let mut hasher = ::jh::Jh224::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct JH256;
impl Hasher for JH256 {
    fn hash_name(&self) -> &'static str {
        "JH-256"
    }

    fn active_bits(&self) -> &'static u32 {
        &256
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ::jh::Digest;
        let mut hasher = ::jh::Jh256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct JH384;
impl Hasher for JH384 {
    fn hash_name(&self) -> &'static str {
        "JH-384"
    }

    fn active_bits(&self) -> &'static u32 {
        &384
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ::jh::Digest;
        let mut hasher = ::jh::Jh384::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct JH512;
impl Hasher for JH512 {
    fn hash_name(&self) -> &'static str {
        "JH-512"
    }

    fn active_bits(&self) -> &'static u32 {
        &512
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ::jh::Digest;
        let mut hasher = ::jh::Jh512::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}
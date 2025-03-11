use super::Hasher;

pub struct Shabal192;
impl Hasher for Shabal192 {
    fn hash_name(&self) -> &'static str {
        "Shabal-192"
    }

    fn active_bits(&self) -> &'static u32 {
        &192
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ::shabal::Digest;
        let mut hasher = ::shabal::Shabal192::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct Shabal224;
impl Hasher for Shabal224 {
    fn hash_name(&self) -> &'static str {
        "Shabal-224"
    }

    fn active_bits(&self) -> &'static u32 {
        &224
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ::shabal::Digest;
        let mut hasher = ::shabal::Shabal224::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct Shabal256;
impl Hasher for Shabal256 {
    fn hash_name(&self) -> &'static str {
        "Shabal-256"
    }

    fn active_bits(&self) -> &'static u32 {
        &256
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ::shabal::Digest;
        let mut hasher = ::shabal::Shabal256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct Shabal384;
impl Hasher for Shabal384 {
    fn hash_name(&self) -> &'static str {
        "Shabal-384"
    }

    fn active_bits(&self) -> &'static u32 {
        &384
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ::shabal::Digest;
        let mut hasher = ::shabal::Shabal384::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct Shabal512;
impl Hasher for Shabal512 {
    fn hash_name(&self) -> &'static str {
        "Shabal-512"
    }

    fn active_bits(&self) -> &'static u32 {
        &512
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ::shabal::Digest;
        let mut hasher = ::shabal::Shabal512::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}
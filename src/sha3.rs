use super::Hasher;

pub struct Sha3_224;
impl Hasher for Sha3_224 {
    fn hash_name(&self) -> &'static str {
        "SHA3-224"
    }

    fn active_bits(&self) -> &'static u32 {
        &224
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use sha3::Digest;
        let mut hasher = sha3::Sha3_224::default();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct Sha3_256;
impl Hasher for Sha3_256 {
    fn hash_name(&self) -> &'static str {
        "SHA3-256"
    }

    fn active_bits(&self) -> &'static u32 {
        &256
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use sha3::Digest;
        let mut hasher = sha3::Sha3_256::default();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct Sha3_384;
impl Hasher for Sha3_384 {
    fn hash_name(&self) -> &'static str {
        "SHA3-384"
    }

    fn active_bits(&self) -> &'static u32 {
        &384
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use sha3::Digest;
        let mut hasher = sha3::Sha3_384::default();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct Sha3_512;
impl Hasher for Sha3_512 {
    fn hash_name(&self) -> &'static str {
        "SHA3-512"
    }

    fn active_bits(&self) -> &'static u32 {
        &512
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use sha3::Digest;
        let mut hasher = sha3::Sha3_512::default();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}
use super::Hasher;

pub struct Sha2_224;
impl Hasher for Sha2_224 {
    fn hash_name(&self) -> &'static str {
        "SHA-224"
    }

    fn active_bits(&self) -> &'static u32 {
        &224
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use sha2::Digest;
        let mut hasher = sha2::Sha224::default();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct Sha2_256;
impl Hasher for Sha2_256 {
    fn hash_name(&self) -> &'static str {
        "SHA-256"
    }

    fn active_bits(&self) -> &'static u32 {
        &256
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use sha2::Digest;
        let mut hasher = sha2::Sha256::default();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct Sha2_384;
impl Hasher for Sha2_384 {
    fn hash_name(&self) -> &'static str {
        "SHA-384"
    }

    fn active_bits(&self) -> &'static u32 {
        &384
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use sha2::Digest;
        let mut hasher = sha2::Sha384::default();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct Sha2_512;
impl Hasher for Sha2_512 {
    fn hash_name(&self) -> &'static str {
        "SHA-512"
    }

    fn active_bits(&self) -> &'static u32 {
        &512
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use sha2::Digest;
        let mut hasher = sha2::Sha512::default();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct Sha2_512Trunc224;
impl Hasher for Sha2_512Trunc224 {
    fn hash_name(&self) -> &'static str {
        "SHA-512/224"
    }

    fn active_bits(&self) -> &'static u32 {
        &224
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use sha2::Digest;
        let mut hasher = sha2::Sha512_224::default();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct Sha2_512Trunc256;
impl Hasher for Sha2_512Trunc256 {
    fn hash_name(&self) -> &'static str {
        "SHA-512/256"
    }

    fn active_bits(&self) -> &'static u32 {
        &256
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use sha2::Digest;
        let mut hasher = sha2::Sha512_256::default();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}
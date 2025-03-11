use super::Hasher;

pub struct Keccak224;
impl Hasher for Keccak224 {
    fn hash_name(&self) -> &'static str {
        "Keccak-224"
    }

    fn active_bits(&self) -> &'static u32 {
        &224
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use sha3::Digest;
        let mut hasher = sha3::Keccak224::default();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct Keccak256;
impl Hasher for Keccak256 {
    fn hash_name(&self) -> &'static str {
        "Keccak-256"
    }

    fn active_bits(&self) -> &'static u32 {
        &256
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use sha3::Digest;
        let mut hasher = sha3::Keccak256::default();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct Keccak384;
impl Hasher for Keccak384 {
    fn hash_name(&self) -> &'static str {
        "Keccak-384"
    }

    fn active_bits(&self) -> &'static u32 {
        &384
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use sha3::Digest;
        let mut hasher = sha3::Keccak384::default();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct Keccak512;
impl Hasher for Keccak512 {
    fn hash_name(&self) -> &'static str {
        "Keccak-512"
    }

    fn active_bits(&self) -> &'static u32 {
        &512
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use sha3::Digest;
        let mut hasher = sha3::Keccak512::default();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

use super::Hasher;

pub struct Groestl224;
impl Hasher for Groestl224 {
    fn hash_name(&self) -> &'static str {
        "Groestl (224bit)"
    }

    fn active_bits(&self) -> &'static u32 {
        &224
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ::groestl::Digest;
        let mut hasher = ::groestl::Groestl224::default();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct Groestl256;
impl Hasher for Groestl256 {
    fn hash_name(&self) -> &'static str {
        "Groestl (256bit)"
    }

    fn active_bits(&self) -> &'static u32 {
        &256
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ::groestl::Digest;
        let mut hasher = ::groestl::Groestl256::default();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct Groestl384;
impl Hasher for Groestl384 {
    fn hash_name(&self) -> &'static str {
        "Groestl (384bit)"
    }

    fn active_bits(&self) -> &'static u32 {
        &384
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ::groestl::Digest;
        let mut hasher = ::groestl::Groestl384::default();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct Groestl512;
impl Hasher for Groestl512 {
    fn hash_name(&self) -> &'static str {
        "Groestl (512bit)"
    }

    fn active_bits(&self) -> &'static u32 {
        &512
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ::groestl::Digest;
        let mut hasher = ::groestl::Groestl512::default();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}
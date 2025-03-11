use super::{Hasher};

pub struct Fsb160;
impl Hasher for Fsb160 {
    fn hash_name(&self) -> &'static str {
        "FSB (160bit)"
    }

    fn active_bits(&self) -> &'static u32 {
        &160
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ::fsb::Digest;
        let mut hasher = ::fsb::Fsb160::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct Fsb224;
impl Hasher for Fsb224 {
    fn hash_name(&self) -> &'static str {
        "FSB (224bit)"
    }

    fn active_bits(&self) -> &'static u32 {
        &224
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ::fsb::Digest;
        let mut hasher = ::fsb::Fsb224::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct Fsb256;
impl Hasher for Fsb256 {
    fn hash_name(&self) -> &'static str {
        "FSB (256bit)"
    }

    fn active_bits(&self) -> &'static u32 {
        &256
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ::fsb::Digest;
        let mut hasher = ::fsb::Fsb256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct Fsb384;
impl Hasher for Fsb384 {
    fn hash_name(&self) -> &'static str {
        "FSB (384bit)"
    }

    fn active_bits(&self) -> &'static u32 {
        &384
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ::fsb::Digest;
        let mut hasher = ::fsb::Fsb384::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct Fsb512;
impl Hasher for Fsb512 {
    fn hash_name(&self) -> &'static str {
        "FSB (512bit)"
    }

    fn active_bits(&self) -> &'static u32 {
        &512
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ::fsb::Digest;
        let mut hasher = ::fsb::Fsb512::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}
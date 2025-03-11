use super::Hasher;

pub struct Gost94CryptoPro;
impl Hasher for Gost94CryptoPro {
    fn hash_name(&self) -> &'static str {
        "GOST94 CryptoPro"
    }

    fn active_bits(&self) -> &'static u32 {
        &256
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use gost94::Digest;
        let mut hasher = gost94::Gost94CryptoPro::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct Gost94s;
impl Hasher for Gost94s {
    fn hash_name(&self) -> &'static str {
        "GOST94 S-box"
    }

    fn active_bits(&self) -> &'static u32 {
        &256
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use gost94::Digest;
        let mut hasher = gost94::Gost94s2015::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct Gost94Test;
impl Hasher for Gost94Test {
    fn hash_name(&self) -> &'static str {
        "GOST94 Test"
    }

    fn active_bits(&self) -> &'static u32 {
        &256
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use gost94::Digest;
        let mut hasher = gost94::Gost94Test::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct Gost94UA;
impl Hasher for Gost94UA {
    fn hash_name(&self) -> &'static str {
        "GOST94 UAPKI"
    }

    fn active_bits(&self) -> &'static u32 {
        &256
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use gost94::Digest;
        let mut hasher = gost94::Gost94UA::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}
use super::Hasher;

pub struct FarmHash32;
impl Hasher for FarmHash32 {
    fn hash_name(&self) -> &'static str {
        "FarmHash (32bit)"
    }

    fn active_bits(&self) -> &'static u32 {
        &32
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let res: u32 = farmhash::hash32(data);
        res.to_be_bytes().to_vec()
    }
}

pub struct FarmHash64;
impl Hasher for FarmHash64 {
    fn hash_name(&self) -> &'static str {
        "FarmHash (64bit)"
    }

    fn active_bits(&self) -> &'static u32 {
        &64
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let res: u64 = farmhash::hash64(data);
        res.to_be_bytes().to_vec()
    }
}

pub struct FarmHashFingerprint32;
impl Hasher for FarmHashFingerprint32 {
    fn hash_name(&self) -> &'static str {
        "FarmHash fingerprint32"
    }

    fn active_bits(&self) -> &'static u32 {
        &32
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let res: u32 = farmhash::fingerprint32(data);
        res.to_be_bytes().to_vec()
    }
}

pub struct FarmHashFingerprint64;
impl Hasher for FarmHashFingerprint64 {
    fn hash_name(&self) -> &'static str {
        "FarmHash fingerprint64"
    }

    fn active_bits(&self) -> &'static u32 {
        &64
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let res: u64 = farmhash::fingerprint64(data);
        res.to_be_bytes().to_vec()
    }
}
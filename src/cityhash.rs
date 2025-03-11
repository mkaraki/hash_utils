use super::Hasher;

pub struct CityHash32;
impl Hasher for CityHash32 {
    fn hash_name(&self) -> &'static str {
        "CityHash (32bit)"
    }

    fn active_bits(&self) -> &'static u32 {
        &32
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let res: u32 = cityhasher::hash(data);
        res.to_be_bytes().to_vec()
    }
}

pub struct CityHash64;
impl Hasher for CityHash64 {
    fn hash_name(&self) -> &'static str {
        "CityHash (64bit)"
    }

    fn active_bits(&self) -> &'static u32 {
        &64
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let res: u64 = cityhasher::hash(data);
        res.to_be_bytes().to_vec()
    }
}
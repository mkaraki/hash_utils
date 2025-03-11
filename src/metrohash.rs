use super::Hasher;

pub struct MetroHash128;
impl Hasher for MetroHash128 {
    fn hash_name(&self) -> &'static str {
        "MetroHash (128bit)"
    }

    fn active_bits(&self) -> &'static u32 {
        &128
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use metrohash::MetroHash128;
        use std::hash::Hasher;
        let mut hasher = MetroHash128::default();
        hasher.write(data);
        let res128 = hasher.finish128();
        let mut res = Vec::new();
        res.extend_from_slice(&res128.0.to_be_bytes());
        res.extend_from_slice(&res128.1.to_be_bytes());
        res
    }
}

pub struct MetroHash64;
impl Hasher for MetroHash64 {
    fn hash_name(&self) -> &'static str {
        "MetroHash (64bit)"
    }

    fn active_bits(&self) -> &'static u32 {
        &64
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use metrohash::MetroHash64;
        use std::hash::Hasher;
        let mut hasher = MetroHash64::default();
        hasher.write(data);
        hasher.finish().to_be_bytes().to_vec()
    }
}
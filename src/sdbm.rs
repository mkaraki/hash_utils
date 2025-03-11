use super::Hasher;

pub struct Sdbm64;
impl Hasher for Sdbm64 {
    fn hash_name(&self) -> &'static str {
        "SDBM (64bit)"
    }

    fn active_bits(&self) -> &'static u32 {
        &64
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use std::hash::Hasher;
        use hashers::oz::SDBMHasher;
        let mut hasher = SDBMHasher::default();
        hasher.write(data);
        hasher.finish().to_be_bytes().to_vec()
    }
}
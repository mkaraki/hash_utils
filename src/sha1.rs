use super::Hasher;

pub struct Sha1;
impl Hasher for Sha1 {
    fn hash_name(&self) -> &'static str {
        "SHA-1"
    }

    fn active_bits(&self) -> &'static u32 {
        &160
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ::sha1::Digest;
        let mut hasher = ::sha1::Sha1::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}
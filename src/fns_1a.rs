use fnv_rs::FnvHasher;
use super::Hasher;

#[cfg(not(feature="hash_fnv_64_minus"))]
pub struct Fnv1a32;
#[cfg(not(feature="hash_fnv_64_minus"))]
impl Hasher for Fnv1a32 {
    fn hash_name(&self) -> &'static str {
        "FNV-1a 32 bit"
    }
    fn active_bits(&self) -> &'static u32 {
        &32
    }
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let mut hasher = fnv_rs::Fnv32::default();
        hasher.update(data);
        hasher.finalize().as_bytes().to_vec()
    }
}

#[cfg(not(feature="hash_fnv_64_minus"))]
pub struct Fnv1a64;
#[cfg(not(feature="hash_fnv_64_minus"))]
impl Hasher for Fnv1a64 {
    fn hash_name(&self) -> &'static str {
        "FNV-1a 64 bit"
    }
    fn active_bits(&self) -> &'static u32 {
        &64
    }
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let mut hasher = fnv_rs::Fnv64::default();
        hasher.update(data);
        hasher.finalize().as_bytes().to_vec()
    }
}

pub struct Fnv1a128;
impl Hasher for Fnv1a128 {
    fn hash_name(&self) -> &'static str {
        "FNV-1a 128 bit"
    }
    fn active_bits(&self) -> &'static u32 {
        &128
    }
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let mut hasher = fnv_rs::Fnv128::default();
        hasher.update(data);
        hasher.finalize().as_bytes().to_vec()
    }
}

pub struct Fnv1a256;
impl Hasher for Fnv1a256 {
    fn hash_name(&self) -> &'static str {
        "FNV-1a 256 bit"
    }
    fn active_bits(&self) -> &'static u32 {
        &256
    }
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let mut hasher = fnv_rs::Fnv256::default();
        hasher.update(data);
        hasher.finalize().as_bytes().to_vec()
    }
}

pub struct Fnv1a512;
impl Hasher for Fnv1a512 {
    fn hash_name(&self) -> &'static str {
        "FNV-1a 512 bit"
    }
    fn active_bits(&self) -> &'static u32 {
        &512
    }
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let mut hasher = fnv_rs::Fnv512::default();
        hasher.update(data);
        hasher.finalize().as_bytes().to_vec()
    }
}

pub struct Fnv1a1024;
impl Hasher for Fnv1a1024 {
    fn hash_name(&self) -> &'static str {
        "FNV-1a 1024 bit"
    }
    fn active_bits(&self) -> &'static u32 {
        &1024
    }
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let mut hasher = fnv_rs::Fnv1024::default();
        hasher.update(data);
        hasher.finalize().as_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(not(feature="hash_fnv_64_minus"))]
    fn fnv_1a_32_hash_validate() {
        use hex::decode;
        let hasher = Fnv1a32;

        assert_eq!(hasher.hash(b"abc"), decode("1a47e90b").unwrap());
        assert_eq!(hasher.hash(b"12345678901234567890123456789012345678901234567890123456789012345678901234567890"), decode("5b52cd65").unwrap());
    }

    #[test]
    #[cfg(not(feature="hash_fnv_64_minus"))]
    fn fnv_1a_64_hash_validate() {
        use hex::decode;
        let hasher = Fnv1a64;

        assert_eq!(hasher.hash(b"abc"), decode("e71fa2190541574b").unwrap());
        assert_eq!(hasher.hash(b"12345678901234567890123456789012345678901234567890123456789012345678901234567890"), decode("95ee3578614f3045").unwrap());
    }

    #[test]
    fn fnv_1a_128_hash_validate() {
        use hex::decode;
        let hasher = Fnv1a128;

        assert_eq!(hasher.hash(b"abc"), decode("a68d622cec8b5822836dbc7977af7f3b").unwrap());
        // ToDo: Add longer test case
    }

    #[test]
    fn fnv_1a_256_hash_validate() {
        use hex::decode;
        let hasher = Fnv1a256;

        assert_eq!(hasher.hash(b"abc"), decode("8b0e658c2f1c837f90d6c7e359de3a1784bd1d30340f770be97fd65817736f4b").unwrap());
        // ToDo: Add longer test case
    }

    #[test]
    fn fnv_1a_512_hash_validate() {
        use hex::decode;
        let hasher = Fnv1a512;

        assert_eq!(hasher.hash(b"abc"), decode("142433ed48a78bb429a7dba8911e8824dcd76c02620000000000001f96475fbd69323ab91bbf83bd3e36fbfd7d0c038b1075dbff4f7a2150e9f28b6e798100d3").unwrap());
        // ToDo: Add longer test case
    }

    #[test]
    fn fnv_1a_1024_hash_validate() {
        use hex::decode;
        let hasher = Fnv1a1024;

        assert_eq!(hasher.hash(b"abc"), decode("000000000001868ce88bd2c7cdc5fa5e52ebb9925ff5ea668dff4576aa4ba65819176ce6b925a8420606e2000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000011d09af071cf00b53007a8e594c73348a3dbb339aead4953fdf93cfff54816f5e2d1e29c8f4f").unwrap());
        // ToDo: Add longer test case
    }
}

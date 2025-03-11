use super::Hasher;

pub struct AsconHash;
impl Hasher for AsconHash {
    fn hash_name(&self) -> &'static str {
        "Ascon"
    }

    fn active_bits(&self) -> &'static u32 {
        &256
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ascon_hash::{AsconHash, Digest};
        let mut hasher = AsconHash::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct AsconAHash;
impl Hasher for AsconAHash {
    fn hash_name(&self) -> &'static str {
        "AsconA"
    }

    fn active_bits(&self) -> &'static u32 {
        &256
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ascon_hash::{AsconAHash, Digest};
        let mut hasher = AsconAHash::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct AsconXof;
impl Hasher for AsconXof {
    fn hash_name(&self) -> &'static str {
        "AsconXof 32byte"
    }

    fn active_bits(&self) -> &'static u32 {
        &256
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ascon_hash::{ ExtendableOutput, Update, XofReader };
        let mut hasher = ascon_hash::AsconXof::default();
        hasher.update(data);
        let mut reader = hasher.finalize_xof();
        let mut output = vec![0; 256];
        reader.read(&mut output);
        output
    }
}

pub struct AsconAXof;
impl Hasher for AsconAXof {
    fn hash_name(&self) -> &'static str {
        "AsconAXof 32byte"
    }

    fn active_bits(&self) -> &'static u32 {
        &256
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use ascon_hash::{ ExtendableOutput, Update, XofReader };
        let mut hasher = ascon_hash::AsconAXof::default();
        hasher.update(data);
        let mut reader = hasher.finalize_xof();
        let mut output = vec![0; 256];
        reader.read(&mut output);
        output
    }
}
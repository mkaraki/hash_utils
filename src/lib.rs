#[cfg(test)]
extern crate hex;

#[cfg(feature="hash_md4")]
mod md4;
#[cfg(feature="hash_md2")]
mod md2;
#[cfg(feature="hash_crc")]
mod crc;
#[cfg(feature="hash_crc_essential")]
mod crc_essential;
#[cfg(feature="hash_unix_cksum")]
mod unix_cksum;
#[cfg(feature="hash_bsd_checksum")]
mod bsd_checksum;
#[cfg(feature="hash_sysv_checksum")]
mod sysv_checksum;
#[cfg(feature="hash_adler32")]
mod adler32;
#[cfg(feature="hash_fletcher_16_plus")]
mod fletcher_16_plus;
#[cfg(feature="hash_fnv_64_minus")]
mod fnv_64_minus;
#[cfg(feature="hash_fnv_1a")]
mod fns_1a;
#[cfg(feature="hash_lrc")]
mod lrc;
#[cfg(feature="hash_md5")]
mod md5;
#[cfg(feature="hash_djb2")]
mod djb2;
#[cfg(feature="hash_sdbm")]
mod sdbm;
#[cfg(feature="hash_lose_lose")]
mod lose_lose;
#[cfg(feature="hash_xxhash")]
mod xxhash;
#[cfg(feature="hash_metrohash")]
mod metrohash;
#[cfg(feature="hash_cityhash")]
mod cityhash;
#[cfg(feature="hash_farmhash")]
mod farmhash_64_minus;
#[cfg(feature="hash_ascon_hash")]
mod ascon;
#[cfg(feature="hash_belt_hash")]
mod belt_hash;
#[cfg(feature="hash_blake2")]
mod blake2;
#[cfg(feature="hash_fsb")]
mod fsb;
#[cfg(feature="hash_gost94")]
mod gost94;
#[cfg(feature="hash_groestl")]
mod groestl;
#[cfg(feature="hash_jh")]
mod jh;
#[cfg(feature="hash_ripemd")]
mod ripemd;
#[cfg(feature="hash_sha1")]
mod sha1;
#[cfg(feature="hash_sha2")]
mod sha2;
#[cfg(feature="hash_sha3")]
mod sha3;
#[cfg(feature="hash_keccak")]
mod keccak;
#[cfg(feature="hash_shabal")]
mod shabal;
#[cfg(feature="hash_skein")]
mod skein;
#[cfg(feature="hash_sm3")]
mod sm3;
#[cfg(feature="hash_streebog")]
mod streebog;
#[cfg(feature="hash_whirlpool")]
mod whirlpool;

use cfg_if::cfg_if;

pub trait Hasher {
    fn hash_name(&self) -> &'static str;
    fn active_bits(&self) -> &'static u32;
    fn hash(&self, data: &[u8]) -> Vec<u8>;
}

pub fn all_hashers() -> Vec<Box<dyn Hasher>> {
    let mut hashers: Vec<Box<dyn Hasher>> = Vec::new();
    cfg_if! {
        if #[cfg(feature="hash_md2")] {
            hashers.push(Box::new(md2::Md2));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_md4")] {
            hashers.push(Box::new(md4::Md4));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_md5")] {
            hashers.push(Box::new(md5::Md5));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_crc")] {
            hashers.append(&mut crc::get_all_crc_hashers());
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_crc_essential")] {
            hashers.push(Box::new(crc_essential::Crc32));
            hashers.push(Box::new(crc_essential::Crc32C));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_unix_cksum")] {
            hashers.push(Box::new(unix_cksum::UnixCksum));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_bsd_checksum")] {
            hashers.push(Box::new(bsd_checksum::BsdChecksum));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_sysv_checksum")] {
            hashers.push(Box::new(sysv_checksum::SysVChecksum));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_adler32")] {
            hashers.push(Box::new(adler32::Adler32));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_fletcher_16_plus")] {
            hashers.push(Box::new(fletcher_16_plus::Fletcher16));
            hashers.push(Box::new(fletcher_16_plus::Fletcher32));
            hashers.push(Box::new(fletcher_16_plus::Fletcher64));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_fnv_64_minus")] {
            hashers.push(Box::new(fnv_64_minus::Fnv0_32));
            hashers.push(Box::new(fnv_64_minus::Fnv0_64));
            hashers.push(Box::new(fnv_64_minus::Fnv1_32));
            hashers.push(Box::new(fnv_64_minus::Fnv1_64));
            hashers.push(Box::new(fnv_64_minus::Fnv1a32));
            hashers.push(Box::new(fnv_64_minus::Fnv1a64));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_fnv_1a")] {
            cfg_if!{
                if #[cfg(not(feature="hash_fnv_64_minus"))] {
                    hashers.push(Box::new(fns_1a::Fnv1a32));
                    hashers.push(Box::new(fns_1a::Fnv1a64));
                }
            }
            hashers.push(Box::new(fns_1a::Fnv1a128));
            hashers.push(Box::new(fns_1a::Fnv1a256));
            hashers.push(Box::new(fns_1a::Fnv1a512));
            hashers.push(Box::new(fns_1a::Fnv1a1024));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_lrc")] {
            hashers.push(Box::new(lrc::LongitudinalRedundancyCheck));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_djb2")] {
            hashers.push(Box::new(djb2::Djb2_64));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_sdbm")] {
            hashers.push(Box::new(sdbm::Sdbm64));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_lose_lose")] {
            hashers.push(Box::new(lose_lose::LoseLose));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_xxhash")] {
            hashers.push(Box::new(xxhash::XxHash32));
            hashers.push(Box::new(xxhash::XxHash64));
            hashers.push(Box::new(xxhash::Xxh3Hash64));
            hashers.push(Box::new(xxhash::Xxh3Hash128));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_metrohash")] {
            hashers.push(Box::new(metrohash::MetroHash64));
            hashers.push(Box::new(metrohash::MetroHash128));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_cityhash")] {
            hashers.push(Box::new(cityhash::CityHash32));
            hashers.push(Box::new(cityhash::CityHash64));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_farmhash_64_minus")] {
            hashers.push(Box::new(farmhash_64_minus::FarmHash32));
            hashers.push(Box::new(farmhash_64_minus::FarmHash64));
            hashers.push(Box::new(farmhash_64_minus::FarmHashFingerprint32));
            hashers.push(Box::new(farmhash_64_minus::FarmHashFingerprint64));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_ascon_hash")] {
            hashers.push(Box::new(ascon::AsconHash));
            hashers.push(Box::new(ascon::AsconAHash));
            hashers.push(Box::new(ascon::AsconXof));
            hashers.push(Box::new(ascon::AsconAXof));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_belt_hash")] {
            hashers.push(Box::new(belt_hash::BeltHash));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_blake2")] {
            hashers.push(Box::new(blake2::Blake2B512));
            hashers.push(Box::new(blake2::Blake2S256));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_fsb")] {
            hashers.push(Box::new(fsb::Fsb160));
            hashers.push(Box::new(fsb::Fsb224));
            hashers.push(Box::new(fsb::Fsb256));
            hashers.push(Box::new(fsb::Fsb384));
            hashers.push(Box::new(fsb::Fsb512));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_gost94")] {
            hashers.push(Box::new(gost94::Gost94CryptoPro));
            hashers.push(Box::new(gost94::Gost94s));
            hashers.push(Box::new(gost94::Gost94Test));
            hashers.push(Box::new(gost94::Gost94UA));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_groestl")] {
            hashers.push(Box::new(groestl::Groestl224));
            hashers.push(Box::new(groestl::Groestl256));
            hashers.push(Box::new(groestl::Groestl384));
            hashers.push(Box::new(groestl::Groestl512));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_jh")] {
            hashers.push(Box::new(jh::JH224));
            hashers.push(Box::new(jh::JH256));
            hashers.push(Box::new(jh::JH384));
            hashers.push(Box::new(jh::JH512));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_ripemd")] {
            hashers.push(Box::new(ripemd::Ripemd128));
            hashers.push(Box::new(ripemd::Ripemd160));
            hashers.push(Box::new(ripemd::Ripemd256));
            hashers.push(Box::new(ripemd::Ripemd320));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_sha1")] {
            hashers.push(Box::new(sha1::Sha1));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_sha2")] {
            hashers.push(Box::new(sha2::Sha2_224));
            hashers.push(Box::new(sha2::Sha2_256));
            hashers.push(Box::new(sha2::Sha2_384));
            hashers.push(Box::new(sha2::Sha2_512));
            hashers.push(Box::new(sha2::Sha2_512Trunc224));
            hashers.push(Box::new(sha2::Sha2_512Trunc256));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_sha3")] {
            hashers.push(Box::new(sha3::Sha3_224));
            hashers.push(Box::new(sha3::Sha3_256));
            hashers.push(Box::new(sha3::Sha3_384));
            hashers.push(Box::new(sha3::Sha3_512));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_keccak")] {
            hashers.push(Box::new(keccak::Keccak224));
            hashers.push(Box::new(keccak::Keccak256));
            hashers.push(Box::new(keccak::Keccak384));
            hashers.push(Box::new(keccak::Keccak512));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_shabal")] {
            hashers.push(Box::new(shabal::Shabal192));
            hashers.push(Box::new(shabal::Shabal224));
            hashers.push(Box::new(shabal::Shabal256));
            hashers.push(Box::new(shabal::Shabal384));
            hashers.push(Box::new(shabal::Shabal512));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_skein")] {
            hashers.push(Box::new(skein::Skein256));
            hashers.push(Box::new(skein::Skein512));
            hashers.push(Box::new(skein::Skein1024));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_sm3")] {
            hashers.push(Box::new(sm3::Sm3));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_streebog")] {
            hashers.push(Box::new(streebog::Streebog256));
            hashers.push(Box::new(streebog::Streebog512));
        }
    }
    cfg_if! {
        if #[cfg(feature="hash_whirlpool")] {
            hashers.push(Box::new(whirlpool::Whirlpool));
        }
    }
    return hashers;
}
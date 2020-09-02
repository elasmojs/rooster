use easy_hasher::easy_hasher;

pub struct Crypto{}

impl Crypto{
    pub fn crc32(input: String) -> String{
        let hash = easy_hasher::crc32(&input);
        return hash.to_hex_string();
    }

    pub fn md5(input: String) -> String{
        let hash = easy_hasher::md5(&input);
        return hash.to_hex_string();
    }

    pub fn sha2(input: String) -> String{
        let hash = easy_hasher::sha256(&input);
        return hash.to_hex_string();
    }

    pub fn sha3(input: String) -> String{
        let hash = easy_hasher::sha3_256(&input);
        return hash.to_hex_string();
    }
}

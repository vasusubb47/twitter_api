use rand::{distributions::Alphanumeric, Rng};
use sha2::{Digest, Sha256};

pub fn genarate_salt(salt_len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(salt_len)
        .map(char::from)
        .collect()
}

pub fn hash_passcode_with_salt(password: String, salt_len: usize) -> String {
    let mut sha = Sha256::new();
    let salt = genarate_salt(salt_len);

    sha.update(password + &salt.to_owned());
    let passcode_hash = sha.finalize();

    let passcode_hash = format!("{:x}", passcode_hash) + ":" + &salt;

    passcode_hash
}

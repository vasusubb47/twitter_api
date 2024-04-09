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

pub fn compare_passcode(password_hash: String, password: String) -> bool {
    let password_parts = password_hash.split(':').collect::<Vec<&str>>();
    let passcode_hash = password_parts[0];
    let passcode_salt = password_parts[1];

    let mut sha = Sha256::new();
    sha.update(password.to_owned() + passcode_salt);
    let user_password_hash = format!("{:x}", sha.finalize());

    passcode_hash == user_password_hash
}

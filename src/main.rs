use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use sha256::digest;

fn encrypt(seedphrase: &str, password: &str) -> String {
    let key = digest(password);
    let mc = new_magic_crypt!(key, 256);
    let encrypted_str = mc.encrypt_str_to_base64(seedphrase);
    encrypted_str
}

fn decrypt(encrypted_seedphrase: &str, password: &str) -> String {
    let key = digest(password);
    let mc = new_magic_crypt!(key, 256);
    let decrypted_str = mc.decrypt_base64_to_string(encrypted_seedphrase).unwrap();
    decrypted_str
}

fn main() {
    let seedphrase = "Insert your seed phrase here";
    let password = "Your password here";

    let encrypted_seedphrase = encrypt(seedphrase, password);

    println!("{}", encrypted_seedphrase);

    let decrypted_seedphrase = decrypt(&encrypted_seedphrase, password);

    println!("{}", decrypted_seedphrase);
}

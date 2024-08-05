use sha2::{Sha256, Digest};

pub fn verify(secret_number: u32, nonce: u32, salt: u32, secret_number_hash: &str){
    println!("\nVerification:");
    println!("Secret number: {}", secret_number);
    println!("Nonce: {}", nonce);
    println!("Salt: {}", salt);
    println!("Hash of secret number (SHA-256): {}", secret_number_hash);

    let verification_str = format!("{}{}{}", secret_number, nonce, salt);
    let mut verification_hasher = Sha256::new();
    verification_hasher.update(verification_str.as_bytes());
    let verification_result = verification_hasher.finalize();
    let verification_hash = hex::encode(verification_result);

    if verification_hash == secret_number_hash {
        println!("Verification successful! The hash matches.");
    } 
    else {
        println!("Verification failed! The hash does not match.");
    }
}
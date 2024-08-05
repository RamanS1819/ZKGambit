use crate::zkp;
use bellman::groth16;
use bls12_381::Bls12;
use colored::*;

pub fn verify(
    params: &groth16::Parameters<Bls12>,
    pvk: &groth16::PreparedVerifyingKey<Bls12>,
    secret_number: u32
){
    println!("\nVerification:");
    println!("Verifying the integrity of the game using ZK proof...");

    // create a proof with actual secret number
    let verification_proof = zkp::create_proof(params, secret_number, secret_number);

    // verify the proof
    let is_valid = zkp::verify_proof(&pvk, &verification_proof);

    if is_valid {
        println!("{}", "Verification successful!".green());
        println!("The game was fair. The secret number exists and the ZKP system is working correctly.");
        println!("Note: The actual secret number was never revealed during this process.");
    }
    else{
        println!("{}", "Verification failed!".red());
        println!("The game was not fair. The secret number does not exist or the ZKP system is not working correctly.");
        println!("There might be an issue with the ZKP system or the game's integrity.");
    }
}
mod verifier;

use std::io;
use rand::rngs::OsRng;
use rand::RngCore;
use sha2::{Sha256, Digest};
use colored::*;

fn main() {
    println!("Guessing Game!!!!\n");


    println!("Enter the amount: ");
    
    let mut x = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");

    let x: u32 = x.trim().parse().expect("Please type a number!");



    let mut rng = OsRng;
    let secret_number = (rng.next_u32() % 5) + 1;
    let nonce: u32 = rng.next_u32();
    let salt: u32 = rng.next_u32();

    let secret_number_str = format!("{}{}{}", secret_number, nonce, salt);                               // Converting the secret number to a string
    let mut hasher = Sha256::new();                                 // Creating a SHA-256 hasher object
    hasher.update(secret_number_str.as_bytes());                                                          // Writing the string data to the hasher
    let result = hasher.finalize();                                      // Finalize the hash and obtain the result as a byte array
    let secret_number_hash = hex::encode(result);                                                // Converting the result to a hexadecimal string

    // println!("Secret number: {}", secret_number);
    println!("Hash (SHA-256): {}", secret_number_hash);             // Printing the hash

    let mut attempts = 0;
    let max_attempts = 2;

    loop{

        attempts += 1;

        if attempts > max_attempts {
            println!("You've used all your attempts!");
            break;
        }


        println!("{}", "Guess the number!".blue());
        
        let mut guess = String::new();

        io::stdin()
            .read_line( &mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        let guess_number_str = format!("{}{}{}", guess, nonce, salt);                  // Converting the guessed number to a string
        let mut hasher1 = Sha256::new();                                                       // Creating a SHA-256 hasher object
        hasher1.update(guess_number_str.as_bytes());                                           // Writing the string data to the hasher
        let guess_result = hasher1.finalize();                                                  // Finalize the hash and obtain the result as a byte array
        let guess_number_hash = hex::encode(guess_result);                        // Converting the result to a hexadecimal string

        // println!("Your guessed number: {}", guess);
        // println!("Hash of your guessed number (SHA-256): {}", guess_number_hash);             // Printing the hash



        if guess_number_hash == secret_number_hash{
            println!("{}", "Hurray!! You guessed it correct".green());
            println!("Secret number: {}", secret_number);
            println!("Hash of secret number (SHA-256): {}", secret_number_hash);
            println!("Your guessed number: {}", guess);
            println!("Hash of your guessed number (SHA-256): {}", guess_number_hash);
            println!("Here is your won prize: {}", x*2);
            break;
        }

        else{
            println!("{}", "Oops!! You guessed it wrong".red());
            // println!("Secret number: {}", secret_number);
            // println!("Hash of secret number (SHA-256): {}", secret_number_hash);  
            // println!("Your guessed number: {}", guess);
            // println!("Hash of your guessed number (SHA-256): {}", guess_number_hash);
            // println!("Here is your lost prize: {}", x/2);
        }

    }




     // Ask user if they want to verify the game
    println!("\nDo you want to verify the integrity of the game? (yes/no):");
    let mut verify_choice = String::new();
    io::stdin().read_line(&mut verify_choice).expect("Failed to read line");
    let verify_choice = verify_choice.trim().to_lowercase();

    if verify_choice == "yes" {
        // Call the verification function from the verifier module
        verifier::verify(secret_number, nonce, salt, &secret_number_hash);
    } 
    else {
        println!("Verification skipped.");
    }

}
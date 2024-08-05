mod verifier;
mod zkp;
mod pyth_integration;

use std::io;
use colored::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Guessing Game With ZK proofs and Pyth random number !!!!\n");


    println!("Enter the amount: ");
    
    let mut x = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");

    let x: u32 = x.trim().parse().expect("Please type a number!");


    // Get random nymber from Pyth
    let secret_number = pyth_integration::get_pyth_random_number().await?;
    
    //Set up the ZKP System
    let (params, pvk) = zkp::setup();

    println!("ZKP System Setup Complete. Ready to play!!");


    let mut attempts = 0;
    let max_attempts = 2;

    loop{

        attempts += 1;

        if attempts > max_attempts {
            println!("You've used all your attempts!");
            break;
        }


        println!("{}", "Guess the number!".blue());
        
        let mut guess_number = String::new();

        io::stdin()
            .read_line( &mut guess_number)
            .expect("Failed to read line");

        let guess: u32 = match guess_number.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };


        // Create a ZKP proof
        let proof = zkp::create_proof(&params, secret_number, guess);

        // Verify the proof
        let is_correct = zkp::verify_proof(&pvk, &proof);

        if is_correct {
            println!("{}", "hurray!! You guessed it correct".green());
            println!("Here is you won prixe: {}", x*2);
            break;
        }
        else {
            println!("{}", "Oops!! You guessed it wrong".red());
        }
    }

     // Ask user if they want to verify the game
    println!("\nDo you want to verify the integrity of the game? (yes/no):");
    let mut verify_choice = String::new();
    io::stdin().read_line(&mut verify_choice).expect("Failed to read line");
    let verify_choice = verify_choice.trim().to_lowercase();

    if verify_choice == "yes" {
        // Call the verification function from the verifier module
        verifier::verify(&params, &pvk, secret_number);
    } 
    else {
        println!("Verification skipped.");
    }

    Ok(())

}
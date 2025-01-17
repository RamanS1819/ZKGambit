use crate::{leaderboard::Leaderboard, pyth_integration, zkp, verifier, token_economics::TokenEconomics};
use std::io;
use std::time::Instant;
use colored::*;

pub async fn game(leaderboard: &mut Leaderboard, token_economics: &mut TokenEconomics) -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter your name:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let name = name.trim().to_string();

    token_economics.add_new_player(&name);

    println!("Choose Difficulty:");
    println!("1. Easy (1-5)");
    println!("2. Medium (1-10)");
    println!("3. Hard (1-20)");

    let mut difficulty = String::new();
    io::stdin()
        .read_line(&mut difficulty)
        .expect("Failed to read line");
    let difficulty: u32 = difficulty.trim().parse().expect("Please type a number!");

    let (range, attempts) = match difficulty {
        1 => (5, 3),
        2 => (10, 5),
        3 => (20, 7),
        _ => {
            println!("Invalid difficulty, defaulting to Easy.");
            (5, 3)
        }
    };


    println!("Enter the amount of token to bet: ");

    let mut x = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");

    let x: u64 = x.trim().parse().expect("Please type a number!");

    if token_economics.get_balance(&name) < x {
        println!("Insufficient balance. Current balance: {}", token_economics.get_balance(&name));
        return Ok(());
    }

    //Deduct the bet amount from the player's balance
    token_economics.transfer(&name, "house", x)?;

    // Get random number from Pyth
    let secret_number = pyth_integration::get_pyth_random_number(range).await?;

    //Set up the ZKP System
    let (params, pvk) = zkp::setup();

    println!("ZKP System Setup Complete. Ready to play!!");

    let mut attempts_left = attempts;
    let start_time = Instant::now();

    loop {
        println!("{}", format!("Attempts left: {}", attempts_left).yellow());
        println!("{}", format!("Guess the number between 1 and {}!", range).blue());


        let mut guess_number = String::new();
        io::stdin()
            .read_line(&mut guess_number)
            .expect("Failed to read line");

        let guess: u32 = match guess_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Create a ZKP proof
        let proof = zkp::create_proof(&params, secret_number, guess);

        // Verify the proof
        let is_correct = zkp::verify_proof(&pvk, &proof);

        if is_correct {
            println!("{}", "Hurray!! You guessed it correct".green());
            let winnings: u64 = x*2;
            println!("You won {} tokens!", winnings);
            match token_economics.house_transfer(&name, winnings) {
                Ok(_) => println!("Tokens transferred successfully to your account"),
                Err(e) => println!("Error transferring tokens: {}", e),
            }
            let duration = start_time.elapsed();
            println!("Time taken: {:?}", duration);
            leaderboard.add_score(name, duration.as_secs() as u32);
            break;
        } else {
            println!("{}", "Oops!! You guessed it wrong".red());
        }

        attempts_left -= 1;
        if attempts_left == 0 {
            println!("You've used all your attempts!");
            println!("You lost {} tokens.", x);
            break;
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
    } else {
        println!("Verification skipped.");
    }

    Ok(())
}
use crate::{leaderboard::Leaderboard, pyth_integration, zkp, verifier};
use std::io;
use std::time::{Instant, Duration};
use colored::*;

pub async fn game(leaderboard: &mut Leaderboard) -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter your name:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let name = name.trim().to_string();

    println!("Choose Difficulty:");
    println!("1. Easy (1-5, 30 seconds)");
    println!("2. Medium (1-10, 45 seconds)");
    println!("3. Hard (1-20, 60 seconds)");

    let mut difficulty = String::new();
    io::stdin()
        .read_line(&mut difficulty)
        .expect("Failed to read line");
    let difficulty: u32 = difficulty.trim().parse().expect("Please type a number!");

    let (range, time_limit) = match difficulty {
        1 => (5, Duration::from_secs(30)),
        2 => (10, Duration::from_secs(45)),
        3 => (20, Duration::from_secs(60)),
        _ => {
            println!("Invalid difficulty, defaulting to Easy.");
            (5, Duration::from_secs(30))
        }
    };

    // Get random number from Pyth
    let secret_number = pyth_integration::get_pyth_random_number(range).await?;

    //Set up the ZKP System
    let (params, pvk) = zkp::setup();

    println!("ZKP System Setup Complete. Ready to play!!");
    println!("You have {:?} seconds to guess the number!", time_limit);

    let start_time = Instant::now();

    loop{
        let elapsed_time = start_time.elapsed();
        if elapsed_time >= time_limit {
            println!("Time's up! You didn't guess in time.");
            break;
        }
        
        println!("{}", format!("Time left: {:?}", time_limit - elapsed_time).yellow());
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
            let duration = start_time.elapsed();
            println!("Time taken: {:?}", duration);
            leaderboard.add_score(name, duration.as_secs() as u32);
            break;

        } else {
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
    } else {
        println!("Verification skipped.");
    }

    Ok(())
}
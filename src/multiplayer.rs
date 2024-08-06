use crate::{leaderboard::Leaderboard, pyth_integration, zkp, verifier};
use std::io;
use std::time::Instant;
use std::collections::HashMap;
use colored::*;

pub async fn game(leaderboard: &mut Leaderboard) -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter the number of players: ");
    let mut num_players = String::new();
    io::stdin()
        .read_line(&mut num_players)
        .expect("Failed to read line");
    let num_players: u32 = num_players.trim().parse().expect("Please type a number!");


    let mut players = HashMap::new();
    let mut total_pool = 0;


    for i in 1..=num_players {
        println!("Player {}, Enter you name:", i);
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");
        let name = name.trim().to_string();


        println!("{}, Enter your bet amount: ", name);
        let mut bet = String::new();
        io::stdin()
            .read_line(&mut bet)
            .expect("Failed to read line");
        let bet: u32 = bet.trim().parse().expect("Please type a number!");


        total_pool += bet;
        players.insert(name, (0, bet));   //(guess, bet)
    }

    println!("Choose Difficulty:");
    println!("1. Easy (1-5)");
    println!("2. Medium (1-10)");
    println!("3. Hard (1-20)");

    let mut difficulty = String::new();
    io::stdin()
        .read_line(&mut difficulty)
        .expect("Failed to read line");
    let difficulty: u32 = difficulty.trim().parse().expect("Please type a number!");

    let range = match difficulty {
        1 => 5,
        2 => 10,
        3 => 20,
        _ => {
            println!("Invalid difficulty, defaulting to Easy.");
            5
        }
    };

    // Get random number from Pyth
    let secret_number = pyth_integration::get_pyth_random_number(range).await?;

    //Set up the ZKP System
    let (params, pvk) = zkp::setup();

    println!("ZKP System Setup Complete. Ready to play!!");

    let start_time = Instant::now();

    for (name, (guess, _)) in players.iter_mut() {
        println!("{}", format!("{}: Guess the number between 1 and {}!", name, range).blue());
        
        let mut guess_number = String::new();
        io::stdin()
            .read_line(&mut guess_number)
            .expect("Failed to read line");

        *guess = guess_number.trim().parse().expect("Please type a number!");
    }

    let mut winners = Vec::new();

    for (name, (guess, _bet)) in players.iter(){
        // Create a ZKP proof
        let proof = zkp::create_proof(&params, secret_number, *guess);

        // Verify the proof
        let is_correct = zkp::verify_proof(&pvk, &proof);

        if is_correct {
            println!("{}", format!("{} guessed it correct", name).green());
            winners.push(name);
        } else {
            println!("{}", format!("{} guessed it wrong", name).red());
        }
    }

    let duration = start_time.elapsed();
    println!("Game Duration: {:?}", duration);

    if winners.is_empty() {
        println!("No one guessed it correctly. The house wins!");
    } else {
        let prize = total_pool / winners.len() as u32;
        for winner in winners {
            println!("{} won {}!", winner, prize);
            leaderboard.add_score(winner.to_string(), duration.as_secs() as u32);
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
use crate::{leaderboard::Leaderboard, pyth_integration, zkp, verifier};
use std::io;
use std::time::Instant;
use colored::*;

pub async fn game(leaderboard: &mut Leaderboard) -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to the Multiplayer Board Game!");
    
    println!("Enter the number of players:");
    let mut num_players = String::new();
    io::stdin().read_line(&mut num_players).expect("Failed to read line");
    let num_players: u32 = num_players.trim().parse().expect("Please enter a valid number");

    let mut players = Vec::new();
    for i in 1..=num_players {
        println!("Enter name for Player {}:", i);
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read line");
        players.push(name.trim().to_string());
    }

    let range = 1000; // Board size
    let max_turns = 10 * num_players; // Adjust max turns based on player count

    // Get random number from Pyth
    let secret_number = pyth_integration::get_pyth_random_number(range).await?;

    //Set up the ZKP System
    let (params, pvk) = zkp::setup();

    println!("ZKP System Setup Complete. Ready to play!!");
    println!("Each player has 10 turns to guess the number between 1 and {}!", range);

    let start_time = Instant::now();
    let mut turns = 0;
    let mut winner = String::new();

    'game_loop: loop {
        for player_name in &players {
            turns += 1;
            println!("{}", format!("Turn {}/{}", turns, max_turns).yellow());
            println!("{}", format!("{}'s turn", player_name).blue());
            println!("{}", format!("Current position on the board: {}", turns).blue());

            println!("Enter your guess:");
            let mut guess_number = String::new();
            io::stdin().read_line(&mut guess_number).expect("Failed to read line");
            let guess: u32 = match guess_number.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input. Skipping turn.");
                    continue;
                }
            };

            // Create a ZKP proof
            let proof = zkp::create_proof(&params, secret_number, guess);

            // Verify the proof
            let is_correct = zkp::verify_proof(&pvk, &proof);

            if is_correct {
                println!("{}", format!("Congratulations, {}! You've reached the correct position!", player_name).green());
                winner = player_name.clone();
                break 'game_loop;
            } else {
                if guess < secret_number {
                    println!("{}", "The correct position is further ahead. Move forward!".yellow());
                } else {
                    println!("{}", "You've gone too far. Move back!".yellow());
                }
            }

            if turns >= max_turns {
                println!("All players have used their turns! Game over.");
                break 'game_loop;
            }
        }
    }

    let duration = start_time.elapsed();
    println!("Game ended. Time taken: {:?}", duration);

    if !winner.is_empty() {
        println!("{} wins!", winner);
        leaderboard.add_score(winner, duration.as_secs() as u32);
    } else {
        println!("No winner this time!");
    }

    // Ask user if they want to verify the game
    println!("\nDo you want to verify the integrity of the game? (yes/no):");
    let mut verify_choice = String::new();
    io::stdin().read_line(&mut verify_choice).expect("Failed to read line");
    let verify_choice = verify_choice.trim().to_lowercase();

    if verify_choice == "yes" {
        verifier::verify(&params, &pvk, secret_number);
    } else {
        println!("Verification skipped.");
    }

    Ok(())
}
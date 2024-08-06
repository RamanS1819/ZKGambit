mod verifier;
mod zkp;
mod pyth_integration;

use std::io;
use colored::*;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Guessing Game With ZK proofs and Pyth random number !!!!\n");

    // Choose game mode
    println!("Choose the game mode:");
    println!("1. Single Player");
    println!("2. Multiplayer");
    let mut game_mode = String::new();
    io::stdin()
        .read_line(&mut game_mode)
        .expect("Failed to read line");
    let game_mode: u32 = game_mode.trim().parse().expect("Please type a number!");

    if game_mode == 1 {
        single_player_game().await?;
    } else if game_mode == 2 {
        multiplayer_game().await?;
    } else {
        println!("Invalid game mode selected. Exiting...");
    }

    Ok(())

}

async fn single_player_game() -> Result<(), Box<dyn std::error::Error>> {
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


    println!("Enter the amount: ");
    
    let mut x = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");

    let x: u32 = x.trim().parse().expect("Please type a number!");


    // Get random nymber from Pyth
    let secret_number = pyth_integration::get_pyth_random_number(range).await?;
    
    //Set up the ZKP System
    let (params, pvk) = zkp::setup();

    println!("ZKP System Setup Complete. Ready to play!!");


    let mut attempts_left = attempts;

    loop{
        println!("{}", format!("Attempts left: {}", attempts_left).blue());
        println!("{}", format!("Guess the number between 1 and {}!", range).blue());
        

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
            println!("{}", "Hurray!! You guessed it correct".green());
            println!("Here is you won prixe: {}", x*2);
            break;
        }
        else {
            println!("{}", "Oops!! You guessed it wrong".red());
        }

        attempts_left -= 1;
        if attempts_left == 0 {
            println!("You've used all your attempts!");
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
    } 
    else {
        println!("Verification skipped.");
    }

    Ok(())

}

async fn multiplayer_game() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter the bumber of players: ");
    let mut num_players = String::new();
    io::stdin()
        .read_line(&mut num_players)
        .expect("Failed to read line");
    let num_players: u32 = num_players.trim().parse().expect("Please type a number!");

    let mut players = HashMap::new();
    let mut total_pool = 0;

    for i in 1..=num_players {
        println!("Player {}, enter you name:", i);
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
        players.insert(name, (0,bet));   //(guess, bet)
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

    if winners.is_empty() {
        println!("No one guessed it correctly. The house wins!");
    } else {
        let prize = total_pool / winners.len() as u32;
        for winner in winners {
            println!("{} won {}!", winner, prize);
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
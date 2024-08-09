mod verifier;
mod zkp;
mod pyth_integration;
mod leaderboard;
mod single_player;
mod multiplayer;
mod time_attack;
mod board_game;
mod mini_games;
mod token_economics;

use std::io;
use leaderboard::Leaderboard;
use token_economics::TokenEconomics;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut leaderboard = Leaderboard::new();
    let mut token_economics = TokenEconomics::new();

    loop {
        println!("Guessing Game With ZK proofs and Pyth random number !!!!\n");

        // Choose game mode
        println!("Choose the game mode:");
        println!("1. Single Player");
        println!("2. Multiplayer");
        println!("3. Time Attack");
        println!("4. Board Game");
        println!("5. Mini-Games");
        println!("6. View Leaderboard");
        println!("7. Token Management");
        println!("8. Exit");

        let mut game_mode = String::new();
        io::stdin()
            .read_line(&mut game_mode)
            .expect("Failed to read line");
        let game_mode: u32 = game_mode.trim().parse().expect("Please type a number!");

        match game_mode {
            1 => single_player::game(&mut leaderboard, &mut token_economics).await?,
            2 => multiplayer::game(&mut leaderboard, &mut token_economics).await?,
            3 => time_attack::game(&mut leaderboard).await?,
            4 => board_game::game(&mut leaderboard).await?,
            5 => mini_games_menu().await?,
            6 => leaderboard.display(),
            7 => token_management_menu(&mut token_economics).await?, // Add this line
            8 => break,
            _ => println!("Invalid mode selected!"),
        }
    }
    Ok(())

}


async fn token_management_menu(token_economics: &mut TokenEconomics) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        println!("\nToken Management Menu:");
        println!("1. Check Balance");
        println!("2. Stake Tokens");
        println!("3. Unstake Tokens");
        println!("4. Use Faucet (if balance is 0)");
        println!("5. Return to Main Menu");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Please type a number!");

        match choice {
            1 => {
                println!("Enter your name:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim();
                let balance = token_economics.get_balance(name);
                let staked = token_economics.get_staked_amount(name);
                println!("Your balance: {} tokens", balance);
                println!("Your staked amount: {} tokens", staked);
            },

            2 => {
                println!("Enter your name:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim();
                println!("Enter amount to stake:");
                let mut amount = String::new();
                io::stdin().read_line(&mut amount).expect("Failed to read line");
                let amount: u64 = amount.trim().parse().expect("Please type a number!");
                match token_economics.stake(name, amount) {
                    Ok(_) => println!("Successfully staked {} tokens", amount),
                    Err(e) => println!("Error: {}", e),
                }
            },
            3 => {
                println!("Enter your name:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim();
                println!("Enter amount to unstake:");
                let mut amount = String::new();
                io::stdin().read_line(&mut amount).expect("Failed to read line");
                let amount: u64 = amount.trim().parse().expect("Please type a number!");
                match token_economics.unstake(name, amount) {
                    Ok(_) => println!("Successfully unstaked {} tokens", amount),
                    Err(e) => println!("Error: {}", e),
                }
            },
            4 => {
                println!("Enter your name:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim();
                match token_economics.faucet(name) {
                    Ok(_) => println!("Successfully received {} tokens from the faucet", token_economics.get_initial_balance()),
                    Err(e) => println!("Error: {}", e),
                }
            },
            5 => break,
            _ => println!("Invalid choice. Please try again."),
        }
    }

    Ok(())
}


async fn mini_games_menu() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        println!("\nMini-Games Menu:");
        println!("1. Coin Flip");
        println!("2. Dice Roll");
        println!("3. High Card");
        println!("4. Return to Main Menu");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Please type a number!");

        match choice {
            1 => mini_games::coin_flip().await?,
            2 => mini_games::dice_roll().await?,
            3 => mini_games::high_card().await?,
            4 => break,
            _ => println!("Invalid choice. Please try again."),
        }
    }

    Ok(())
}

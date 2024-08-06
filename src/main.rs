mod verifier;
mod zkp;
mod pyth_integration;
mod leaderboard;
mod single_player;
mod multiplayer;
mod time_attack;
mod board_game;
mod mini_games;

use std::io;
use leaderboard::Leaderboard;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut leaderboard = Leaderboard::new();

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
        println!("7. Exit");

        let mut game_mode = String::new();
        io::stdin()
            .read_line(&mut game_mode)
            .expect("Failed to read line");
        let game_mode: u32 = game_mode.trim().parse().expect("Please type a number!");

        match game_mode {
            1 => single_player::game(&mut leaderboard).await?,
            2 => multiplayer::game(&mut leaderboard).await?,
            3 => time_attack::game(&mut leaderboard).await?,
            4 => board_game::game(&mut leaderboard).await?,
            5 => mini_games_menu().await?,
            6 => leaderboard.display(),
            7 => break,
            _ => println!("Invalid mode selected!"),
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

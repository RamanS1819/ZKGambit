use crate::pyth_integration;
use std::io;
use colored::*;

pub async fn coin_flip() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Coin Flip!");
    println!("Guess: Heads (H) or Tails (T)");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess = guess.trim().to_uppercase();

    let random_number = pyth_integration::get_pyth_random_number(2).await?;
    let result = if random_number == 1 { "H" } else { "T" };

    println!("The coin shows: {}", if result == "H" { "Heads" } else { "Tails" });

    if guess == result {
        println!("{}", "You guessed correctly!".green());
    } else {
        println!("{}", "Sorry, wrong guess.".red());
    }

    Ok(())
}

pub async fn dice_roll() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Dice Roll!");
    println!("Guess a number between 1 and 6:");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    let result = pyth_integration::get_pyth_random_number(6).await?;

    println!("The dice shows: {}", result);

    if guess == result {
        println!("{}", "You guessed correctly!".green());
    } else {
        println!("{}", "Sorry, wrong guess.".red());
    }

    Ok(())
}

pub async fn high_card() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to High Card!");
    println!("You'll draw a card, and then the computer will draw a card.");
    println!("The highest card wins. Aces are high.");

    let player_card = pyth_integration::get_pyth_random_number(13).await?;
    println!("You drew: {}", card_name(player_card));

    println!("Press Enter for the computer to draw a card...");
    let mut _input = String::new();
    io::stdin().read_line(&mut _input).expect("Failed to read line");

    let computer_card = pyth_integration::get_pyth_random_number(13).await?;
    println!("Computer drew: {}", card_name(computer_card));

    if player_card > computer_card {
        println!("{}", "You win!".green());
    } else if player_card < computer_card {
        println!("{}", "Computer wins!".red());
    } else {
        println!("{}", "It's a tie!".yellow());
    }

    Ok(())
}

fn card_name(card: u32) -> String {
    match card {
        1 => "Ace".to_string(),
        11 => "Jack".to_string(),
        12 => "Queen".to_string(),
        13 => "King".to_string(),
        _ => card.to_string(),
    }
}
use std::collections::HashMap;

pub struct TokenEconomics {
    balances: HashMap<String, u64>,
    staked_amounts: HashMap<String, u64>,
    total_supply: u64,
    initial_balance: u64,
    house_balance: u64,
}

impl TokenEconomics {
    pub fn new() -> Self {
        let mut economics = TokenEconomics {
            balances: HashMap::new(),
            staked_amounts: HashMap::new(),
            total_supply: 1_000_000, // Initial supply of 1 million tokens
            initial_balance: 100, // Initial balance of 100 tokens for each player
            house_balance: 1_000_000, // Initial balance of 1 million tokens for the house
        };
        economics.balances.insert("house".to_string(), economics.house_balance);
        economics
    }

    pub fn get_balance(&mut self, player: &str) -> u64 {
        if !self.balances.contains_key(player) {
            self.add_new_player(player);
        }
        *self.balances.get(player).unwrap_or(&0)
    }

    pub fn add_new_player(&mut self, player: &str) {
        if !self.balances.contains_key(player) {
            self.mint(player, self.initial_balance);
        }
    }

    pub fn faucet(&mut self, player: &str) -> Result<(), &'static str> {
        if self.get_balance(player) == 0 {
            self.mint(player, self.initial_balance);
            Ok(())
        } else {
            Err("Faucet is only available for players with 0 balance")
        }
    }

    pub fn mint(&mut self, player: &str, amount: u64) {
        *self.balances.entry(player.to_string()).or_insert(0) += amount;
        self.total_supply += amount;
    }

    pub fn transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<(), &'static str> {
        // let from_balance = self.get_balance(from);
        if self.balances.get(from).unwrap_or(&0) < &amount {
            return Err("Insufficient balance");
        }
        *self.balances.entry(from.to_string()).or_insert(0) -= amount;
        *self.balances.entry(to.to_string()).or_insert(0) += amount;
        Ok(())
    }

    pub fn house_transfer(&mut self, to: &str, amount: u64) -> Result<(), &'static str> {
        if self.house_balance < amount {
            self.mint("house", amount); // Ensure house always has enough tokens
        }
        self.transfer("house", to, amount)
    }

    pub fn stake(&mut self, player: &str, amount: u64) -> Result<(), &'static str> {
        if self.balances.get(player).unwrap_or(&0) < &amount {
            return Err("Insufficient balance to stake");
        }
        *self.balances.entry(player.to_string()).or_insert(0) -= amount;
        *self.staked_amounts.entry(player.to_string()).or_insert(0) += amount;
        Ok(())
    }

    pub fn unstake(&mut self, player: &str, amount: u64) -> Result<(), &'static str> {
        if self.staked_amounts.get(player).unwrap_or(&0) < &amount {
            return Err("Insufficient staked amount");
        }
        *self.staked_amounts.entry(player.to_string()).or_insert(0) -= amount;
        *self.balances.entry(player.to_string()).or_insert(0) += amount;
        Ok(())
    }

    pub fn get_initial_balance(&self) -> u64 {
        self.initial_balance
    }

    pub fn get_staked_amount(&self, player: &str) -> u64 {
        *self.staked_amounts.get(player).unwrap_or(&0)
    }

    pub fn calculate_staking_reward(&self, player: &str) -> u64 {
        let staked_amount = self.get_staked_amount(player);
        // Simple reward calculation: 1% of staked amount
        staked_amount / 100
    }
}
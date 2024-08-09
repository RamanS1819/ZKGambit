use std::str::FromStr;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use pyth_sdk_solana::PriceFeed;
use solana_sdk::account_info::AccountInfo;

pub async fn get_pyth_random_number(range: u32) -> Result<u32, Box<dyn std::error::Error>> {
    let client = RpcClient::new("https://api.devnet.solana.com".to_string());

    // Pyth price account for SOL/USD on devnet
    let pyth_price_key = Pubkey::from_str("J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix")?;

    let mut price_account = client.get_account(&pyth_price_key)?;
    
    // Create an AccountInfo struct
    let account_info = AccountInfo::new(
        &pyth_price_key,
        false,
        false,
        &mut price_account.lamports,
        &mut price_account.data,
        &price_account.owner,
        price_account.executable,
        price_account.rent_epoch,
    );

    let price_feed: PriceFeed = pyth_sdk_solana::load_price_feed_from_account_info(&account_info)?;
    let current_price = price_feed.get_price_unchecked();


    // Use multiple fields for better randomness
    let mut random_seed = current_price.price.abs() as u64 
                    + current_price.conf as u64 
                    + current_price.publish_time as u64;
    
    // Generate a random number using rejection sampling
    let range_u64 = range as u64;
    let max_valid = u64::MAX - (u64::MAX % range_u64);
    
    loop {
        if random_seed < max_valid {
            return Ok(((random_seed % range_u64) + 1) as u32);
        }
        // If we're here, we need to generate a new random seed
        random_seed = random_seed.wrapping_mul(1103515245).wrapping_add(12345);

    }
    // Generate a random number
    // Ok(((random_seed % range as u64) + 1) as u32)
}

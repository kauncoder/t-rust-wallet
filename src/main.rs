mod wallet;
use anyhow::Result;
use web3::types::{Address};
use std::{ str::FromStr,env};

//use std::env;
#[tokio::main]
async fn main() -> Result<()> {

    dotenv::dotenv().ok();

    let (secret_key, pub_key) = wallet::generate_keypair();

    let pub_address = wallet::get_public_address(&pub_key);

    let w = wallet::MyWallet {
        secret_key: secret_key.to_string(),
        public_key: pub_key.to_string(),
        public_address: format!("{:?}", pub_address),
    };

    println!("wallet details: {:?}", &w);
    let endpoint = env::var("INFURA_SEPOLIA")?;
    let web3_con = wallet::connect_to_eth(&endpoint).await?;

    let block_number = web3_con.eth().block_number().await?;
    println!("block#: {}", &block_number);
    let balance = wallet::get_balance(&w,&web3_con).await?;
    println!("balance left: {}", &balance);

    let dest_addr = Address::from_str("0x2.....[get address with sepolia eth here]")?;

    let transaction = wallet::create_txn(dest_addr,1)?;
    let sk = wallet::get_secret_key(&w)?;   //not really needed right now since sk is accessible in the code (yikes!)
    let transact_hash = wallet::sign_and_send(&web3_con, transaction,&sk).await?;
    println!("transaction hash: {:?}", transact_hash);
    Ok(())

}

use secp256k1::{
    rand::{rngs, SeedableRng},
    PublicKey, SecretKey,
};
use anyhow::{Result};
use std::str::FromStr;
use tiny_keccak::keccak256;
use web3::{
    transports::http,
    types::{Address,TransactionParameters, U256, H256},
    Web3,
};

#[derive(Debug)]
pub struct MyWallet {
    pub secret_key: String,
    pub public_key: String,
    pub public_address: String,
}

pub fn generate_keypair() -> (SecretKey, PublicKey) {
    let secp = secp256k1::Secp256k1::new();
    let mut rng = rngs::StdRng::seed_from_u64(1234);    //bad bad bad seed
    secp.generate_keypair(&mut rng)
}

pub fn get_public_address(public_key: &PublicKey) -> Address {
    let public_key = public_key.serialize_uncompressed();
    debug_assert_eq!(public_key[0], 0x04);
    let hash = keccak256(&public_key[1..]);
    Address::from_slice(&hash[12..])
}

pub fn get_secret_key(mw:&MyWallet) -> Result<SecretKey> {
    let secret_key = SecretKey::from_str(&mw.secret_key)?;
    Ok(secret_key)
}

pub async fn connect_to_eth(url: &str) -> Result<Web3<http::Http>> {
    let transport = web3::transports::Http::new(url)?;
    Ok(Web3::new(transport))
}

pub async fn get_balance(mw:&MyWallet,web3_connection: &Web3<http::Http>) -> Result<U256> {
    let wallet_address = Address::from_str(&mw.public_address)?;
    let balance = web3_connection.eth().balance(wallet_address, None).await?;
    Ok(balance)
}

pub fn create_txn(dest: Address, amount: usize) -> Result<TransactionParameters> {
    let tp = TransactionParameters {
        to: Some(dest),
        value: U256::exp10(amount),
        ..Default::default()
    };
    Ok(tp)
}

pub async fn sign_and_send(web3: &Web3<http::Http>,txn_object: TransactionParameters,secret_key: &SecretKey) -> Result<H256> {
    let signed = web3
        .accounts()
        .sign_transaction(txn_object, secret_key)
        .await?;
    let txn_result = web3
        .eth()
        .send_raw_transaction(signed.raw_transaction)
        .await?;
    Ok(txn_result)
}

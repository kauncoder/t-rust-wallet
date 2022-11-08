# T-rust-Wallet

What is this?

A most basic Crypto-Wallet built using Rust for the Sepolia ETH testnet. Very low-quality in terms of security, so please dont use for anything serious.

What does this do?
This creates a command-line wallet that can interact with wallets on an ETH testnet (Sepolia in our case)
The code will create a most basic wallet that can access the testnet endpoint via Infura. It will then transfer the defined amount to another wallet (we use metamask to connect the destination address on Sepolia for this). 

Why make this?
Short term goal is to convince budding devs that building things like this in Rust is much easier than they might believe.
Long term goal is to have a repo of cryptographic protocols, mostly threshold/multi-signatures, for the wallet side of things.
The endgoal is not to develop sophisticated code for the crypto industry but to provide an opening platform for beginners in crypto to understand how to write cryptographic protocols.

Why Rust?
Because I hate myself. And also because it is the dopest programming language of all time.

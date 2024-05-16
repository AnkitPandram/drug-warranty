# Drug Authenticity Verification on Stellar using Soroban

This project leverages the Stellar blockchain and the Soroban smart contract platform to establish a robust system for verifying the authenticity of pharmaceutical drugs throughout their supply chain.

## Vision

The pharmaceutical industry faces significant challenges with counterfeit drugs entering the supply chain, posing serious health risks to consumers. This project aims to combat this problem by providing a decentralized, transparent, and tamper-proof system for tracking and verifying the authenticity of drugs.

## Key Features

- **Unique Identification:** Each drug package is assigned a unique identifier (e.g., serial number, QR code, RFID tag) to facilitate tracking.
- **Blockchain Registration:** Upon manufacture, the drug's identifier, origin, batch information, and an optional hash of its composition are registered on the Stellar blockchain via the Soroban smart contract.
- **Verification at Every Stage:** Stakeholders throughout the supply chain (manufacturers, distributors, pharmacies) can scan the identifier to instantly verify the drug's authenticity using the smart contract.
- **Flagging Counterfeits:** Authorized entities (e.g., regulators) can flag potentially counterfeit drugs, preventing their further distribution.

## Smart Contract Functionality

- **`register_drug(identifier, batch_info, hash)`:**  Registers a new drug on the blockchain.
- **`verify_drug(identifier)`:** Returns `true` if the drug is authentic and not flagged, `false` otherwise.
- **`flag_drug(identifier)`:** Flags a drug as potentially counterfeit.

## Project Setup

1. **Prerequisites:**
   - Rust toolchain (install from [rustup.rs](https://rustup.rs/))
   - Soroban CLI (install instructions: [Soroban Docs](https://soroban.stellar.org/))
   - Stellar account funded with XLM

   Soroban contract deploy --wasm ./target/wasm32-unknown-unknown/release/drug_auth.wasm --network testnet

## How to clone the project:
   git clone https://your-repository-url/drug_waranty.git
   cd drug_waranty
   cargo build --release

   Thank You



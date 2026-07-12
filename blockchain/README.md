# Blockchain Notes

Notes on blockchain fundamentals, consensus, and a minimal Rust implementation.

## Table of Contents

- [Web Evolution](#web-evolution)
- [Core Concepts](#core-concepts)
- [Consensus Mechanisms](#consensus-mechanisms)
- [Platforms](#platforms)
- [Key Terminology](#key-terminology)
- [Data Model](#data-model)
- [Rust Implementation](#rust-implementation)

## Web Evolution

| Version | Description |
|---------|-------------|
| Web1 | Static HTML pages |
| Web2 | Social, interactive platforms |
| Web3 | Blockchain (smart contracts) — crypto (BTC, ETH), NFTs, DeFi, DAOs; identity, storage, ownership (NFTs) |

## Core Concepts

- **Block** — A batch of transactions. Contains transaction details, its own hash (generated from the previous hash), and the previous block's hash.
- **Blockchain** — A chain of blocks. Immutable, acts as a public ledger.
- **Smart Contract** — Processing code that enforces rules (e.g., only allow transfers where `amount < 100`).
- **Proof** — Real work required to add a new block (e.g., generate a hash that starts with `00` using a nonce). Costly to produce but cheap to verify — temporary proof of effort.
- **Mining** — The process of searching for a valid new block.

### How a Transaction Gets Added

1. A transaction is broadcast to the network.
2. A miner collects pending transactions and packs them into a block.
3. The miner starts mining (finding valid proof).
4. The block is shared with the network.
5. Other nodes verify the block.
6. The miner receives a reward.

## Consensus Mechanisms

| Mechanism | Role | Example Chain |
|-----------|------|---------------|
| Proof of Work (PoW) | Miner | Bitcoin |
| Proof of Stake (PoS) | Validator | Ethereum |

## Platforms

| Platform | Language | Strength |
|----------|----------|----------|
| Ethereum | Solidity | Reliability, smart contracts |
| Solana | Rust | Speed, DApps, NFTs |

## Key Terminology

- **Transaction** — Can represent anything: a post on a DApp, a file stored on IPFS, a transfer of value, etc.
- **Consensus Machine** — PoW, PoS.
- **Merkle Tree** — A tree of hashes used to efficiently and securely verify data integrity.
- **Node** — A computer participating in the network.
- **ZKP (Zero-Knowledge Proof)** — Proving knowledge without revealing it. Wallet analogy: a locker with a private key (to sign) and a public key (to verify).
- **Assets** — Crypto (BTC, ETH) or tokens (digital assets):
  - **Fungible** — Non-unique (interchangeable).
  - **Non-fungible (NFT)** — Unique.

## Data Model

```
struct Transaction (from, to, amount)
struct Block (transactions, index, timestamp, hash, prev_hash)

impl Block::new:
    calculate hash from (index, timestamp, transactions, prev_hash)
```

## Rust Implementation

A minimal blockchain skeleton in Rust.

```rust
struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

struct Block {
    index: u64,
    timestamp: u64,
    transactions: Vec<Transaction>,
    hash: String,
    prev_hash: String,
}

impl Block {
    fn new(index: u64, transactions: Vec<Transaction>, prev_hash: String) -> Self {
        let timestamp = current_timestamp();
        let hash = calculate_hash(&data);
        Self {
            index,
            timestamp,
            transactions,
            hash,
            prev_hash,
        }
    }
}

struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        let genesis = Block::new(0, vec![], String::from("0"));
        Self {
            chain: vec![genesis],
        }
    }

    fn add_block(&mut self, transactions: Vec<Transaction>) {
        let last_block = self.chain.last().unwrap();
        let new_block = Block::new(
            last_block.index + 1,
            transactions,
            last_block.hash.clone(),
        );
        self.chain.push(new_block);
    }
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn calculate_hash(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn main() {
    let mut my_blockchain = Blockchain::new();

    let tx1 = Transaction {
        from: String::from("Alice"),
        to: String::from("Bob"),
        amount: 50,
    };

    let tx2 = Transaction {
        from: String::from("Bob"),
        to: String::from("Charlie"),
        amount: 30,
    };

    my_blockchain.add_block(vec![tx1]);
    my_blockchain.add_block(vec![tx2]);

    my_blockchain.print_chain();
}
```

> **Note:** This is illustrative/rough code — `data` in `calculate_hash(&data)` and `print_chain()` are referenced but not defined; they'd need to be implemented for this to compile.

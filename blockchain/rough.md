-> web1 - html
-> web2 - social
-> web3 - blockchain(smart contract)-crypto(btc,eth),NFTs,DeFi,DAOs; idetity, storage, ownership(NFTs)

----------------------------------------------------------------------------------------------------------

-> Block - batch of transaction - contains transc details, hash(gen using prev hash), prev hash
-> Blockchain - chain of block (non changeable) - public ledger
-> smart contract - rules - processing code - example: only allow small transfer(amt<100)
-> proof - to add new block, do real work -> example: gen hash until starts with 00(nonce-special num) - costly but temp proof
-> mining - digging to find new block

-> blockchain - broadcasted - miner collects transc - pack them into block - start mining - share - other verify - miner gets reward
-> miner - proof of work - bitcoin
-> validaror - proof of stake - ethereum

-----------------------------------------------------------------------------------------------------------

-> ethereum : solidity - smart contract - reliability
-> solana : rust - Dapps, NFTs - speed

-----------------------------------------------------------------------------------------------------------

-> Transc can be anything : post on DApps, storing file on IPFS
-> Consensus machine : pow , pos
-> merkel tree: tree of hashes 
-> node : computer
-> zkp : zero knowledge proof - wallet:locker (keys[private to sign, public to verify])
-> assets: crypto(btc,eth) or token-digital assets(fungible: non-unique, nf: unique)

------------------------------------------------------------------------------------------------------------

-> struct transc (from, to, amt)
-> struct block (transc, index, ts, hash, prevhash)
-> impl new : calc hash (index, ts, transc, prevhash)

------------------------------------------------------------------------------------------------------------

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

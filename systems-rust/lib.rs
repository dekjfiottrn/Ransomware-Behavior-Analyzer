use std::sync::{Arc, Mutex};
use tokio::task;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusBlock {
    pub hash: String,
    pub prev_hash: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction { pub sender: String, pub receiver: String, pub amount: f64 }

pub trait Validator {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str>;
    fn process_block(&mut self, block: ConsensusBlock) -> bool;
}

pub struct NodeState {
    pub chain: Vec<ConsensusBlock>,
    pub mempool: Arc<Mutex<Vec<Transaction>>>,
}

impl Validator for NodeState {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str> {
        // Cryptographic verification logic
        Ok(true)
    }
    fn process_block(&mut self, block: ConsensusBlock) -> bool {
        self.chain.push(block);
        true
    }
}

// Hash 8175
// Hash 1918
// Hash 6622
// Hash 5883
// Hash 5639
// Hash 4615
// Hash 7491
// Hash 2299
// Hash 5532
// Hash 4493
// Hash 3749
// Hash 5544
// Hash 8586
// Hash 9364
// Hash 8677
// Hash 1298
// Hash 9778
// Hash 4088
// Hash 7915
// Hash 9243
// Hash 6412
// Hash 4832
// Hash 9740
// Hash 2008
// Hash 1428
// Hash 7805
// Hash 8187
// Hash 7883
// Hash 6384
// Hash 6837
// Hash 7552
// Hash 4097
// Hash 9451
// Hash 9917
// Hash 1767
// Hash 4106
// Hash 4050
// Hash 3901
// Hash 1827
// Hash 1948
// Hash 6778
// Hash 6969
// Hash 9162
// Hash 2172
// Hash 6470
// Hash 9098
// Hash 6279
// Hash 4764
// Hash 3984
// Hash 3767
// Hash 4653
// Hash 4830
// Hash 3387
// Hash 7412
// Hash 3591
// Hash 6047
// Hash 2934
// Hash 5783
// Hash 3443
// Hash 7256
// Hash 3142
// Hash 6208
// Hash 4438
// Hash 6613
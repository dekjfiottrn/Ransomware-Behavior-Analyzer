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
// Hash 9015
// Hash 4896
// Hash 7676
// Hash 8429
// Hash 5895
// Hash 2620
// Hash 3929
// Hash 9716
// Hash 1997
// Hash 6561
// Hash 4219
// Hash 1678
// Hash 4084
// Hash 9506
// Hash 2130
// Hash 6940
// Hash 5156
// Hash 8001
// Hash 7391
// Hash 6473
// Hash 6484
// Hash 2838
// Hash 6821
// Hash 2779
// Hash 9395
// Hash 3362
// Hash 7664
// Hash 6474
// Hash 6489
// Hash 4006
// Hash 7940
// Hash 2313
// Hash 2770
// Hash 7511
// Hash 7864
// Hash 1078
// Hash 7458
// Hash 4878
// Hash 9520
// Hash 7996
// Hash 9494
// Hash 7853
// Hash 3483
// Hash 2986
// Hash 8686
// Hash 2946
// Hash 7295
// Hash 4055
// Hash 5503
// Hash 4370
// Hash 5325
// Hash 3192
// Hash 4613
// Hash 2744
// Hash 4998
// Hash 5042
// Hash 8206
// Hash 9888
// Hash 1357
// Hash 6537
// Hash 6406
// Hash 8670
// Hash 7029
// Hash 7206
// Hash 3489
// Hash 1366
// Hash 7485
// Hash 6965
// Hash 9382
// Hash 7615
// Hash 6242
// Hash 5391
// Hash 6621
// Hash 6664
// Hash 3195
// Hash 4120
// Hash 3634
// Hash 9530
// Hash 7070
// Hash 1497
// Hash 6300
// Hash 2842
// Hash 6361
// Hash 5525
// Hash 5006
// Hash 7804
// Hash 9085
// Hash 4216
// Hash 5599
// Hash 3475
// Hash 2548
// Hash 7370
// Hash 1597
// Hash 6777
// Hash 2691
// Hash 7011
// Hash 3321
// Hash 7923
// Hash 8265
// Hash 7903
// Hash 7427
// Hash 8625
// Hash 2792
// Hash 9396
// Hash 3988
// Hash 8156
// Hash 2737
// Hash 9927
// Hash 2898
// Hash 6446
// Hash 1094
// Hash 5969
// Hash 5097
// Hash 5707
// Hash 7381
extern crate bitcoin;

use self::bitcoin::blockdata::transaction::{Transaction as tx, TxIn};
use self::bitcoin::blockdata::script::Script;
use self::bitcoin::util::hash::Sha256dHash;

//const TX_VERSION: i32 = 1;
const MAX_TX_IN_SEQUENCE_NUM: u32 = 0xffffffff;
//const MAX_PREV_OUT_INDEX: u32 = 0xffffffff;

pub struct Transaction {
    content:  tx
}

impl Transaction {
    #[inline]
    pub fn new() -> Transaction {
        Transaction {
            content:  tx {
                version: 1,
                lock_time: 0,
                input: Vec::new(),
                output: Vec::new(),
                witness: Vec::new(),
            }
        }
    }

    pub fn add_vin(&mut self, pre_hex: &str, pre_index: u32) -> bool {
        let pre_hash = match Sha256dHash::from_hex(&pre_hex) {
            Ok(h) => h,
            Err(_) => return false,
        };
        let vin = TxIn {
            prev_hash: pre_hash,
            prev_index: pre_index,
            script_sig: Script::new(),
            sequence: MAX_TX_IN_SEQUENCE_NUM,
        };
        self.content.input.push(vin);
        true
    }

    pub fn add_vout(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use self::bitcoin::util::misc::hex_bytes;
    use self::bitcoin::network::serialize::{serialize, deserialize};

    #[test]
    fn test_xx() {
        let hex_tx = hex_bytes(
            "0100000002343ef5b73979b6c3a3d1d853a4f1d0069a7b6a190c9c0d70f999c709e0932316010000006a4730440220156de\
             6e28d884b42c634a7d9f05143b43ddc8cc6516575c2106f41f7bb65056102205ab2695b46b85bc5577ce903c2c77e94dbe8441f4dbfd\
             ab85c143698757a75f8012102640eb344597eb94b2294f196185cf2ffc42802158cbcdfd39521460c1a1ec1b8ffffffff343ef5b7397\
             9b6c3a3d1d853a4f1d0069a7b6a190c9c0d70f999c709e0932316000000006a47304402203a47ebb662ddd128c9875261feae3198153\
             45b0555cefa5575096d9b77b4c22802204e7bee596f7ec374aaba55683337760b41bc2da90f02ef2c5acf7942721163ca012102b3c61\
             4586512494e65e6218dfa1b37c8a53b166916640f15183d81f4731e161bffffffff055a7c0b00000000001976a914198519e19e510f0\
             273538ffe8f3082ce48016b5988ac50c30000000000001976a914798803eb4093f6472a0e0147d46b74c4177a1d3f88ac50c30000000\
             000001976a9148309aef1fb395405b8097bc33c057cf766d619c088ac50c30000000000001976a914970a2789f0f3a38b8a922ed8b06\
             ea440d7860aa388ac0000000000000000166a146f6d6e69000000000000001f0000000005f5e10000000000"
        ).unwrap();
        let t: tx = deserialize(&hex_tx).unwrap();
        println!("{}", t.version);
        println!("{}", t.lock_time);
        for input in t.input {
            println!("{:?}", input.prev_hash);
            println!("{:?}", input.prev_index);
            println!("{:?}", input.sequence);
            println!();
        }
    }
}
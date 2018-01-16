extern crate pbr;
extern crate time;
extern crate secp256k1;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;
use self::pbr::ProgressBar;
use self::time::now;
use super::super::bitcoin;
use self::secp256k1::{Secp256k1, ContextFlag};


pub fn bitcoin_key_address(path: &Path, num: i32) {
    let file = File::create(path).unwrap();
    let mut out = BufWriter::new(file);

    let mut pb = ProgressBar::new(num as u64);
    pb.format("[=>-]");
    let mut secp: Secp256k1 = Secp256k1::with_caps(ContextFlag::SignOnly);

    let begin = now();
    for _ in 1..num + 1 {
        let (key, address) = bitcoin::key::generate_key_address(&mut secp);
        let line = key + " " + &address + "\n";
        out.write_all(line.as_bytes())
            .ok()
            .expect("fail to write line");
        pb.inc();
    }
    pb.finish();
    out.flush()
        .ok()
        .expect("fail to write to file");
    let end = now();
    println!("generate {}, {:?}", num, end - begin);
}

pub fn sample() {
    let mut tx = bitcoin::transaction::Transaction::new();
    let pre_hex = "162393e009c799f9700d9c0c196a7b9a06d0f1a453d8d1a3c3b67939b7f53e34";
    let result = tx.add_vin(pre_hex, 1);
    println!("{:?}", result)
}
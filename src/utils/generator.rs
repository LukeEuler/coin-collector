extern crate pbr;
extern crate time;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;
use self::pbr::ProgressBar;
use self::time::now;
use super::super::bitcoin;


pub fn bitcoin_key_address(path: &Path, num: i32) {
    let file = File::create(path).unwrap();
    let mut out = BufWriter::new(file);

    let mut pb = ProgressBar::new(num as u64);
    pb.format("[=>-]");

    let begin = now();
    for _ in 1..num + 1 {
        let (key, address) = bitcoin::key::generate_key_address();
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
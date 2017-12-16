extern crate coin_collector;

use coin_collector::utils;
use std::path::Path;

fn main() {
    let path = Path::new("/tmp/keys.bitcoin");
    utils::generator::bitcoin_key_address(path, 1000);
}

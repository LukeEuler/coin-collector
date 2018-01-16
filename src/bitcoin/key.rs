extern crate bitcoin;
extern crate secp256k1;
extern crate rand;
extern crate time;

use self::bitcoin::util::address::{Privkey};
use self::bitcoin::util::base58::{FromBase58, ToBase58};
use self::secp256k1::{Secp256k1, ContextFlag};
use self::secp256k1::key::SecretKey;
use self::rand::os::OsRng;
use self::bitcoin::network::constants::Network::Bitcoin;

pub fn generate_key_address(secp: &mut Secp256k1) -> (String, String) {
    let mut rng = match OsRng::new() {
        Ok(o) => { o }
        Err(e) => panic!(e)
    };
    let secret_key = SecretKey::new(&secp, &mut rng);
    let private_key = Privkey::from_key(Bitcoin, secret_key, true);

    let address = match private_key.to_address(&secp) {
        Ok(a) => { a }
        Err(e) => panic!(e)
    };
    (private_key.to_base58check(), address.to_base58check())
}

pub fn key_address_check(wif: &str, address: &str) -> bool {
    let secp: Secp256k1 = Secp256k1::with_caps(ContextFlag::SignOnly);
    let private_key: Privkey = match FromBase58::from_base58check(wif) {
        Ok(p) => { p }
        Err(e) => panic!(e)
    };
    let addr = match private_key.to_address(&secp) {
        Ok(a) => { a }
        Err(e) => panic!(e)
    };
    addr.to_base58check() == address
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_address_check() {
        let wif = "Kyi9EqDEB7fg17ApNyCVbPSp8KxgoXUXXEC2XFCbkfmhhUpT4rFx";
        let address = "13KwLph2FM1V6Ugo693B95q9XCxr1aXn8Z";

        assert!(key_address_check(wif, address), "wif: {}, address: {}", wif, address)
    }
}
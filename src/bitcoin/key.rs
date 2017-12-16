extern crate bitcoin;
extern crate secp256k1;
extern crate rand;

use self::bitcoin::util::address::{Privkey, Address};
use self::bitcoin::util::base58::{FromBase58, ToBase58};
use self::secp256k1::{Secp256k1, ContextFlag};
use self::secp256k1::key::SecretKey;
use self::rand::os::OsRng;
use self::bitcoin::network::constants::Network::Bitcoin;

pub fn generate_key_address() -> (String, String) {
    let mut rng: OsRng;
    match OsRng::new() {
        Ok(o) => {
            rng = o;
        }
        Err(e) => {
            panic!(e)
        }
    }
    let secp: Secp256k1 = Secp256k1::with_caps(ContextFlag::SignOnly);
    let secret_key = SecretKey::new(&secp, &mut rng);
    let private_key = Privkey::from_key(Bitcoin, secret_key, true);

    let address: Address;
    match private_key.to_address(&secp) {
        Ok(a) => {
            address = a;
        }
        Err(e) => {
            panic!(e)
        }
    }
    (private_key.to_base58check(), address.to_base58check())
}

pub fn key_address_check(wif: &str, address: &str) -> bool {
    let secp: Secp256k1 = Secp256k1::with_caps(ContextFlag::SignOnly);
    let private_key: Privkey;
    match FromBase58::from_base58check(wif) {
        Ok(p) => {
            private_key = p;
        }
        Err(e) => {
            panic!(e)
        }
    }
    let addr: Address;
    match private_key.to_address(&secp) {
        Ok(a) => {
            addr = a;
        }
        Err(e) => {
            panic!(e)
        }
    }
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
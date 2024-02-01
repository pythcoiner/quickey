extern crate bitcoin;
extern crate bip39;

use std::str::FromStr;
use bitcoin::bip32::{Xpub, DerivationPath};
use bitcoin::network::Network;
use bip39::{Mnemonic};
use bitcoin::bip32;
use bitcoin::secp256k1::{Secp256k1};

fn main() {
    let mnemonic = Mnemonic::generate(12).unwrap();
    println!("Mnemonic: {}", &mnemonic);

    let seed = &mnemonic.to_seed("");

    let der = "/44'/0'/0'";

    let derivation_path = DerivationPath::from_str(&format!("m{}", &der)).unwrap();

    let secp = Secp256k1::new();

    let xpriv = bip32::Xpriv::new_master(Network::Bitcoin, seed)
        .unwrap()
        .derive_priv(&secp, &derivation_path)
        .unwrap();

    let xpub = Xpub::from_priv(&secp, &xpriv);

    println!("[{}{}]{}", &xpub.parent_fingerprint, der, &xpub.to_string());
}

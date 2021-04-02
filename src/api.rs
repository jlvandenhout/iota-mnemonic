use std::{convert::TryInto, str::FromStr};

use iota::Ed25519Address;
use crypto::{
    hashes::{
        Digest,
        blake2b::Blake2b256,
    },
    keys::{
        bip39::{mnemonic_to_seed, wordlist},
        slip10::{Chain, Curve, Seed},
    },
    signatures::ed25519::SecretKey,
    utils::rand::fill,
};


// Create a cryptographically secure random mnemonic using BIP-0039
// See: https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki
pub fn random_mnemonic() -> String {
    let mut random_data = [0u8; 32];
    fill(&mut random_data).unwrap();

    wordlist::encode(&random_data, &wordlist::ENGLISH).unwrap()
}


// Derive the private key from the mnemonic, the account index and the address index using SLIP-0010
// See: https://github.com/satoshilabs/slips/blob/master/slip-0010.md
pub fn private_key_from_mnemonic(mnemonic: &String, account_index: u32, address_index: u32) -> String {
    let mut seed = [0u8; 64];
    mnemonic_to_seed(mnemonic.as_str(), &"", &mut seed);

    let curve = Curve::Ed25519;
    let chain = Chain::from_u32_hardened(vec![44, 4218, account_index, false as u32, address_index]);

    let bytes = Seed::from_bytes(&seed)
        .derive(curve, &chain).unwrap()
        .secret_key().unwrap()
        .to_le_bytes();
    hex::encode(bytes)
}


// Derive the public key from the private key
pub fn public_key_from_private_key(private_key: &String) -> String {
    let bytes = hex::decode(private_key).unwrap().try_into().unwrap();
    let key = SecretKey::from_le_bytes(bytes).unwrap();

    let bytes = key.public_key().to_compressed_bytes();
    hex::encode(bytes)
}


// Derive the address by hashing the public key using BLAKE2b256
pub fn ed25519_address_from_public_key(public_key: &String) -> String {
    let bytes = hex::decode(public_key).unwrap();
    let hash = Blake2b256::digest(&bytes);
    Ed25519Address::new(hash.into()).to_string()
}


// Derive the human readable address using Bech32
pub fn bech32_address_from_ed25519_address(ed25519_address: &String, hrp: &String) -> String {
    let address = Ed25519Address::from_str(&ed25519_address.as_str()).unwrap();
    address.to_bech32(hrp.as_str())
}
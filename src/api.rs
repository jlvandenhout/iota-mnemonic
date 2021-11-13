use std::{convert::TryInto, str::FromStr};

use crypto::{
    hashes::{blake2b::Blake2b256, Digest},
    keys::{
        bip39::{mnemonic_to_seed, wordlist},
        slip10::{Chain, Curve, Seed},
    },
    signatures::ed25519::SecretKey,
    utils::rand::fill,
};
use iota_client::bee_message::address::Address;
use iota_client::bee_message::address::Ed25519Address;

// Create a cryptographically secure random mnemonic using BIP-0039
// See: https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki
pub fn random_mnemonic() -> String {
    let mut random_bytes = [0u8; 32];
    fill(&mut random_bytes).unwrap();

    wordlist::encode(&random_bytes, &wordlist::ENGLISH).unwrap()
}

// Derive the Bech32 seed from the BIP-0039 mnemonic
pub fn seed_from_mnemonic(mnemonic: &str) -> String {
    let mut seed_bytes = [0u8; 64];
    mnemonic_to_seed(mnemonic, "", &mut seed_bytes);
    hex::encode(seed_bytes)
}

// Derive the private key from the Bech32 seed, the account index and the address index using SLIP-0010
// See: https://github.com/satoshilabs/slips/blob/master/slip-0010.md
pub fn private_key_from_seed(seed: &str, account_index: u32, address_index: u32) -> String {
    let seed_bytes: [u8; 64] = hex::decode(seed).unwrap().try_into().unwrap();

    let curve = Curve::Ed25519;
    let chain =
        Chain::from_u32_hardened(vec![44, 4218, account_index, false as u32, address_index]);

    let private_key_bytes = Seed::from_bytes(&seed_bytes)
        .derive(curve, &chain)
        .unwrap()
        .secret_key()
        .to_bytes();
    hex::encode(private_key_bytes)
}

// Derive the public key from the private key
pub fn public_key_from_private_key(private_key: &str) -> String {
    let private_key_bytes = hex::decode(private_key).unwrap().try_into().unwrap();
    let private_key = SecretKey::from_bytes(private_key_bytes);

    let public_key_bytes = private_key.public_key().to_bytes();
    hex::encode(public_key_bytes)
}

// Derive the address by hashing the public key using BLAKE2b256
pub fn ed25519_address_from_public_key(public_key: &str) -> String {
    let public_key_bytes = hex::decode(public_key).unwrap();
    let hash = Blake2b256::digest(&public_key_bytes);
    Ed25519Address::new(hash.into()).to_string()
}

// Derive the human readable address using Bech32
pub fn bech32_address_from_ed25519_address(ed25519_address: &str, hrp: &str) -> String {
    let address = Address::Ed25519(Ed25519Address::from_str(ed25519_address).unwrap());
    address.to_bech32(hrp)
}

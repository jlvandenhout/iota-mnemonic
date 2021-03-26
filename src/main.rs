use iota::{Bech32Address, Ed25519Address};
use crypto::{
    hashes::{
        Digest,
        blake2b::Blake2b256,
    },
    keys::{
        bip39::{mnemonic_to_seed, wordlist},
        slip10::{Chain, Curve, Seed},
    },
    signatures::ed25519::{PublicKey, SecretKey},
    utils::rand::fill,
};


// Create a cryptographically secure random mnemonic using Bip39.
fn random_mnemonic() -> String {
    let mut random_data = [0u8; 32];
    fill(&mut random_data).unwrap();

    wordlist::encode(&random_data, &wordlist::ENGLISH).unwrap()
}


// Derive the private key from the mnemonic, the account index and the address index using Slip10.
fn private_key_from_mnemonic(mnemonic: &str, account_index: u32, address_index: u32) -> SecretKey {
    let mut seed = [0u8; 64];
    mnemonic_to_seed(&mnemonic, &"", &mut seed);

    let curve = Curve::Ed25519;
    let chain = Chain::from_u32_hardened(vec![44, 4218, account_index, false as u32, address_index]);
    Seed::from_bytes(&seed)
        .derive(curve, &chain).unwrap()
        .secret_key().unwrap()
}


// Derive the public key from the private key using Ed25519.
fn public_key_from_private_key(private_key: SecretKey) -> PublicKey {
    private_key.public_key()
}


// Derive the address by hashing the public key using BLAKE2b256.
fn ed25519_address_from_public_key(public_key: PublicKey) -> Ed25519Address {
    let hash = Blake2b256::digest(&public_key.to_compressed_bytes());
    Ed25519Address::new(hash.into())
}


// Derive the human readable address using Bech32.
fn bech32_address_from_ed25519_address(ed25519_address: Ed25519Address, hrp: &str) -> Bech32Address {
    ed25519_address.to_bech32(hrp).into()
}


fn main() {
    let mnemonic = random_mnemonic();
    println!("Mnemonic: {}", mnemonic);

    let private_key = private_key_from_mnemonic(mnemonic.as_str(), 0, 0);
    println!("Private Key: {}", hex::encode(private_key.to_le_bytes()));

    let public_key = public_key_from_private_key(private_key);
    println!("Public Key: {}", hex::encode(public_key.to_compressed_bytes()));

    let ed25519_address = ed25519_address_from_public_key(public_key);
    println!("Ed25519 Address: {}", ed25519_address);

    let bech32_address = bech32_address_from_ed25519_address(ed25519_address, "iota");
    println!("Bech32 Address: {}", bech32_address);
}
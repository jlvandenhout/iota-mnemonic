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


// Create a cryptographically secure random mnemonic using BIP-0039
// See: https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki
pub fn random_mnemonic() -> String {
    let mut random_data = [0u8; 32];
    fill(&mut random_data).unwrap();

    wordlist::encode(&random_data, &wordlist::ENGLISH).unwrap()
}


// Derive the private key from the mnemonic, the account index and the address index using SLIP-0010
// See: https://github.com/satoshilabs/slips/blob/master/slip-0010.md
pub fn private_key_from_mnemonic(mnemonic: &str, account_index: u32, address_index: u32) -> SecretKey {
    let mut seed = [0u8; 64];
    mnemonic_to_seed(&mnemonic, &"", &mut seed);

    let curve = Curve::Ed25519;
    let chain = Chain::from_u32_hardened(vec![44, 4218, account_index, false as u32, address_index]);
    Seed::from_bytes(&seed)
        .derive(curve, &chain).unwrap()
        .secret_key().unwrap()
}


// Derive the public key from the private key
pub fn public_key_from_private_key(private_key: SecretKey) -> PublicKey {
    private_key.public_key()
}


// Derive the address by hashing the public key using BLAKE2b256
pub fn ed25519_address_from_public_key(public_key: PublicKey) -> Ed25519Address {
    let hash = Blake2b256::digest(&public_key.to_compressed_bytes());
    Ed25519Address::new(hash.into())
}


// Derive the human readable address using Bech32
pub fn bech32_address_from_ed25519_address(ed25519_address: Ed25519Address, hrp: &str) -> Bech32Address {
    ed25519_address.to_bech32(hrp).into()
}
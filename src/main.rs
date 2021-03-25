use iota::{
    Seed,
    Address
};
use crypto::keys::bip39;
use iota_client::api::GetAddressesBuilder;

#[tokio::main]
async fn main() {
    let mut mnemonic_seed = [0u8; 64];
    bip39::mnemonic_to_seed(
        &"social wolf hungry label salute hover sudden rain disease upgrade throw quick amazing clinic night",
        &"",
        &mut mnemonic_seed,
    );
    let seed = Seed::from_bytes(&mnemonic_seed);
    let addresses = GetAddressesBuilder::new(&seed)
        .with_bech32_hrp(String::from("atoi"))
        .with_account_index(0)
        .with_range(0..1)
        .finish()
        .await
        .unwrap();

    for address in addresses {
        let addr = Address::try_from_bech32(&address).expect("No valid Address");
        println!("Ed25519 Address: {:?}", addr);
        println!("Bech32 Address: {}", address);
    }
}
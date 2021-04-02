mod api;
use clap;


fn main() {
    let app = clap::app_from_crate!()
        .arg(
            clap::Arg::new("mnemonic")
                .short('m')
                .long("mnemonic")
                .about("Sets a custom mnemonic")
                .takes_value(true),
        );
    let matches = app.get_matches();

    let mnemonic: String;

    if let Some(mnemonic_argument) = matches.value_of("mnemonic") {
        mnemonic = String::from(mnemonic_argument);
    } else {
        mnemonic = api::random_mnemonic();
    }
    println!("Mnemonic: {}", mnemonic);

    let private_key = api::private_key_from_mnemonic(mnemonic.as_str(), 0, 0);
    println!("Private Key: {}", hex::encode(private_key.to_le_bytes()));

    let public_key = api::public_key_from_private_key(private_key);
    println!("Public Key: {}", hex::encode(public_key.to_compressed_bytes()));

    let ed25519_address = api::ed25519_address_from_public_key(public_key);
    println!("Ed25519 Address: {}", ed25519_address);

    let bech32_address = api::bech32_address_from_ed25519_address(ed25519_address, "atoi");
    println!("Bech32 Address: {}", bech32_address);
}
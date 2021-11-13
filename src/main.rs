mod api;
use clap;


fn main() {
    let app = clap::app_from_crate!()
        .arg(
            clap::Arg::new("mnemonic")
                .long("mnemonic")
                .about("A BIP-0039 mnemonic (defaults to a random mnemonic).")
                .takes_value(true),
        )
        .arg(
            clap::Arg::new("hrp")
                .long("hrp")
                .about("The Bech32 human readable part (defaults to 'atoi').")
                .takes_value(true),
        )
        .arg(
            clap::Arg::new("account")
                .long("account")
                .about("The account index (defaults to 0).")
                .takes_value(true),
        )
        .arg(
            clap::Arg::new("address")
                .long("address")
                .about("The address index (defaults to 0).")
                .takes_value(true),
        );
    let matches = app.get_matches();

    let mnemonic = matches.value_of_t("mnemonic").unwrap_or(api::random_mnemonic());
    let seed = api::seed_from_mnemonic(&mnemonic);
    let hrp = matches.value_of_t("hrp").unwrap_or(String::from("atoi"));
    let account_index = matches.value_of_t("account").unwrap_or(0);
    let address_index = matches.value_of_t("address").unwrap_or(0);

    println!("\nINPUT");
    println!("BIP-0039 mnemonic: {}", mnemonic);
    println!("Hex encoded binary seed: {}", seed);

    println!("\nCONFIGURATION");
    println!("Bech32 human readable part: {}", hrp);
    println!("Account index: {}", account_index);
    println!("Address index: {}", address_index);

    let private_key = api::private_key_from_seed(&seed, account_index, address_index);
    let public_key = api::public_key_from_private_key(&private_key);
    let ed25519_address = api::ed25519_address_from_public_key(&public_key);
    let bech32_address = api::bech32_address_from_ed25519_address(&ed25519_address, &hrp);

    println!("\nOUTPUT");
    println!("Hex encoded private key: {}", private_key);
    println!("Hex encoded public key: {}", public_key);
    println!("Ed25519 address: {}", ed25519_address);
    println!("Bech32 address: {}", bech32_address);
}
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

    let mnemonic = matches.value_of_t("mnemonic").unwrap_or(api::random_mnemonic());
    println!("Mnemonic: {}", mnemonic);

    let private_key = api::private_key_from_mnemonic(mnemonic, 0, 0);
    println!("Private Key: {}", private_key);

    let public_key = api::public_key_from_private_key(private_key);
    println!("Public Key: {}", public_key);

    let ed25519_address = api::ed25519_address_from_public_key(public_key);
    println!("Ed25519 Address: {}", ed25519_address);

    let bech32_address = api::bech32_address_from_ed25519_address(ed25519_address, String::from("atoi"));
    println!("Bech32 Address: {}", bech32_address);
}
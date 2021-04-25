# mnemonic

A tool to create random [BIP-0039](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) mnemonics and derive private keys, public keys, Ed25519 addresses and [Bech32](https://github.com/bitcoin/bips/blob/master/bip-0173.mediawiki) addresses from mnemonics, account indexes and address indexes.

## Prerequisites
To use the tool, you need the following:
- [Rust](https://www.rust-lang.org/tools/install)
- (Optional) An IDE that supports Rust autocompletion. We recommend [Visual Studio Code](https://code.visualstudio.com/Download) with the [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) extension

We also recommend updating Rust to the [latest stable version](https://github.com/rust-lang/rustup.rs#keeping-rust-up-to-date):

```bash
rustup update stable
```

## How to build
```bash
git clone https://github.com/jlvandenhout/iota-mnemonic
cd mnemonic
cargo build
```

## How to run
To create a random mnemonic and derive its keys and addresses, run:
```bash
cargo run
```

To see configuration options, run:
```bash
cargo run -- --help
```


## Todo
- [x] Give the option to provide a mnemonic instead of randomly creating one.
- [x] Give the option to provide a custom bech32 human readable part (or switch between test and mainnet).
- [x] Give the option to provide the account and address indexes instead of hardcoding 0.

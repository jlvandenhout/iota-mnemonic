# mnemonic

A tool to create a random [BIP-0039](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) mnemonic and derive its private key, public key, Ed25519 address and [Bech32](https://github.com/bitcoin/bips/blob/master/bip-0173.mediawiki) address.

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
git clone https://github.com/jlvandenhout/mnemonic
cd mnemonic
cargo build
```

## How to run
```bash
cargo run
```

## Todo
- [ ] Give the option to provide a mnemonic instead of randomly creating one.
- [ ] Give the option to provide the account and address indexes instead of hardcoding 0.
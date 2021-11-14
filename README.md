# IOTA Mnemonic

A tool to create random [BIP-0039](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) mnemonics and derive binary seeds, private keys, public keys, Ed25519 addresses and [Bech32](https://github.com/bitcoin/bips/blob/master/bip-0173.mediawiki) addresses from mnemonics, account indexes and address indexes.

| ⚠️ This repository was created for testing purposes only ⚠️                                                                                                                                                                                                                                     |
| ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| To manage your funds, please use [recommended tools](https://www.iota.org/get-started/secure-iota) or the [wallet libraries](https://github.com/iotaledger/wallet.rs). Never use an online seed generator. If you really want to, at least randomly change some of the generated data yourself. |

## Prerequisites

To use the tool, you need to install [Rust](https://www.rust-lang.org/tools/install).

We also recommend updating Rust to the [latest stable version](https://github.com/rust-lang/rustup.rs#keeping-rust-up-to-date):

```bash
rustup update stable
```

## How to build

```bash
git clone https://github.com/jlvandenhout/iota-mnemonic
cd iota-mnemonic
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

## Version history

### v0.2.0

- Changed the commandline output to include the Hex encoded binary seed representation.

### v0.1.0

- Added the option to provide a mnemonic instead of randomly creating one.
- Added the option to provide a custom bech32 human readable part.
- Added the option to provide the account and address indexes instead of hardcoding 0.

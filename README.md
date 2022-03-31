# Asterix-Node
[Asterix](https://asterix.network/) is a proof-of-stake smart contract
blockchain which will bridge the gap between real-world sectors and the "Internet of Blockchains".

Setup:
Install system dependencies:
- Linux: `sudo apt install cmake pkg-config libssl-dev git clang libclang-dev`
- Mac: `brew install cmake pkg-config openssl git llvm`

Install Asterix dependencies:

```
curl https://sh.rustup.rs -sSf | sh
rustup install nightly-2021-03-01
rustup default nightly-2021-03-01
rustup target add wasm32-unknown-unknown --toolchain nightly-2021-03-01
```

Build Asterix:

```
cargo build --release
```
To start up the Asterix node and connect to the Celestial testnet, run:
```
./target/release/asterix --chain=staging --name <INSERT_NAME>
```

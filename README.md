# Faze Token - ERC20 Compliant

This repository contains an erc20 compliant smart contract written in rust. This smart contract can be deployed to any substrate compatible blockchain with contracts pallet.

## Prerequisites

- rust (nightly - requires nightly channel as we will will using some features that are not available over the stable channel)
- ink (substrate's rust compatible smart contracts eDSL)

## Building

Builds a .wasm, metadata.json and .contract binary that can be deployed straight to blockchain.

To build a binary, run

`cargo +nightly contract build`

## Testing

Run unit tests with,

`cargo +nightly test`

## Read More

This smart contract was created as part of the substrate's ink! workshop, found [here](https://substrate.dev/substrate-contracts-workshop/#/).

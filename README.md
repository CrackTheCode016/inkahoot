# Ink! Quizzes

This is a demosntration of a simple quiz system built using [ink! Wasm smart contracts](https://use.ink/).  It is meant to be used with a Substrate node instance that implements the `contracts` pallet.

The primary idea behind this is to allow for anyone to check their answers using this contract, ideally in a multiple choice format.  Checking answers is free - adding new questions, registering as a user, and tracking progress is not (as it costs fees/gas).

## Terminology

- `Quiz` - an entity that defines the actors (educators versus registered users) and a collection of `Question`(s) and answers.
- `Question` - a string of text representing the question, and the answer represented as a hash.  For now, we're just using `String` to store the question text.
- `PowerLevel` - the level of authority, or access.  There are currently two types:
    - `Educator` - Has the ability to add new questions and answers
    - `User` - Has the ability to register and track their progress via the contract (TODO)

## Building and Running

**Requirements**

- [`cargo-contract`](https://github.com/paritytech/cargo-contract)
- [`Rust / Cargo`](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [`substrate-contracts-node`](https://use.ink/getting-started/setup#installing-substrate-contracts-node)


### Building

```sh
cargo contract build
```

### Testing

```sh
cargo test
```

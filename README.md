# readme

a re-implement of enter the gungeon with bevy engine

## Current Progress

you can see it in http://xhslink.com/VH5qWq

## Build

- rustc 1.69.0
- rustc 1.71.0-nightly

## Run

first you need to generate the demo level file with this command:

```shell
cargo run --package world_generator --bin world_generator
```

then you can run the game with this command:

```shell
cargo run --package enter-the-gungeon --bin enter-the-gungeon --release RUST_BACKTRACE=1
```

## Test

it will run all the test

```shell
cargo test --all
```

## Bench

it will run all the benchmark, notice it should run with nightly channel

```shell
cargo +nightly bench --all
```
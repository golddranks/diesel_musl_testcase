#!/bin/sh
docker run -it --rm -v $PWD:/workdir -v ~/.cargo/git:/root/.cargo/git -v ~/.cargo/registry:/root/.cargo/registry golddranks/rust_musl_nightly_docker:latest cargo build --release --target=x86_64-unknown-linux-musl -vv

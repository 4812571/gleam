# Build Stage
FROM ghcr.io/evanrichter/cargo-fuzz:latest AS builder

## Add source code to the build stage.
ADD . /src
WORKDIR /src

# Compile the fuzzers.
RUN cd compiler-core/fuzz && cargo fuzz build

# Package Stage
FROM ubuntu:latest
COPY --from=builder /src/compiler-core/fuzz/target/x86_64-unknown-linux-gnu/release/fuzz_* /fuzz/
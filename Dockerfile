FROM rustlang/rust:nightly AS builder
RUN rustup target add wasm32-wasi

WORKDIR /build
COPY . .
RUN cargo build --bin responder-server --target wasm32-wasi --release

FROM scratch
COPY --from=builder /build/target/wasm32-wasi/release/responder-server.wasm /responder-server.wasm
ENTRYPOINT [ "responder-server.wasm" ]

FROM rust:1.58 as builder

WORKDIR /tmp-server
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src
RUN cargo build

FROM debian:buster-slim
COPY --from=builder /tmp-server/target/debug/server /bin/server
CMD ["/bin/server"]
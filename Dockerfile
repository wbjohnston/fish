FROM rust:1.50 as builder



WORKDIR /usr/src/fish
RUN cargo init --bin --name fish
COPY Cargo.lock ./Cargo.lock
COPY Cargo.toml ./Cargo.toml
RUN cargo build --release
# RUN cargo build
RUN rm src/*.rs
# build and cache dependencies 
COPY src ./src
RUN rm ./target/release/deps/fish*
RUN cargo build --release

FROM debian:buster
COPY --from=builder /usr/src/fish/target/release/fish /usr/bin/fish
EXPOSE 8080
ENTRYPOINT [ "/usr/bin/fish"]

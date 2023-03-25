FROM rust:1.61.0
WORKDIR /usr/src/relic_rust
COPY . .
RUN cargo install --path .
CMD ["relic_rust"]
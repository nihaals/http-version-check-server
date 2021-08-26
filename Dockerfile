FROM rust:1 AS builder
WORKDIR /src
COPY . .
RUN cargo build --release

FROM rust:1
COPY --from=builder /src/target/release/http-version-check-server /server
CMD ["/server"]

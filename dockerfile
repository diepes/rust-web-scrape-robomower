FROM rust:1.69.0 as build-env
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/rust-web-scrape /
CMD ["./rust-web-scrape"]

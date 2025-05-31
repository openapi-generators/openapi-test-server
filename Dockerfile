FROM rust:1.87 AS builder

WORKDIR app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12 AS runtime
COPY --from=builder /app/target/release/openapi-test-server /app
CMD ["./app"]
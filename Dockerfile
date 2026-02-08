# Builder stage
FROM rust:1.89 as builder

WORKDIR /app
COPY . .

RUN cargo build --release

# Runtime stage
FROM gcr.io/distroless/cc-debian12

COPY --from=builder /app/target/release/molt-config /app/molt-bot

WORKDIR /app

CMD ["./molt-bot"]

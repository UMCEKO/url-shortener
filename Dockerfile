FROM rust as builder
WORKDIR /app
COPY . .
RUN cargo build --release


FROM rust as runner
WORKDIR /app
COPY --from=builder /app/target/release/url-shortener /app/
COPY migrations /app/
CMD ["./url-shortener"]

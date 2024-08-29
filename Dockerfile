FROM rust:1.49
LABEL authors="vaughnw128"

COPY . .

RUN cargo build --release

CMD ["./target/release/mc-status-rs"]
FROM rust:latest as builder

WORKDIR /app

# Copÿ source code
COPY Cargo.* ./
COPY ./src ./src

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /app

# name = "text_prediction_api"
COPY --from=builder /app/target/release/text_prediction_api .

EXPOSE $PORT

CMD ["./text_prediction_api"]

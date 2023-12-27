FROM rust:latest

WORKDIR /app

COPY ./Cargo.toml .
COPY ./src ./src
COPY .env .


RUN cargo build --release

EXPOSE 8080

ENV ROCKET_ENV=production

CMD ["sh", "-c", "cargo run"]

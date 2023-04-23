# build rust app nightly
FROM rustlang/rust:nightly as builder
LABEL stage=rustbuilder

WORKDIR /app

COPY . .

RUN cargo install --path .

# run rust app
FROM debian:buster-slim as api

EXPOSE 8000

COPY --from=builder /usr/local/cargo/bin/bibliotheca /usr/local/bin/bibliotheca

CMD ["bibliotheca"]
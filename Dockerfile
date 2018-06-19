FROM rust:latest as builder
COPY . .
RUN cargo install
CMD ["actix_web_test"]

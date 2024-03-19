FROM rust:1.76.0
WORKDIR /app
COPY . .
COPY . .

RUN apt update && apt install -y protobuf-compiler
RUN cargo build --release --package api

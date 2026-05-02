FROM rust:1.95-slim

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY . .

RUN cargo build --release -p dc-gossip

EXPOSE 8001/udp

CMD ["./target/release/dc-gossip"]

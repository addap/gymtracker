FROM rust:1.65 as builder

RUN rustup target add wasm32-unknown-unknown && cargo install trunk

WORKDIR /usr/src/gymtracker
COPY . .
RUN cargo install --path gt-backend
RUN cd gt-frontend/ && trunk build

FROM debian:bullseye-slim

WORKDIR /app/gymtracker

# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/gt-backend /usr/local/bin/gt-backend
COPY --from=builder /usr/src/gymtracker/gt-frontend/dist /app/gymtracker/dist

# CMD ["gt-backend"]
FROM rust:1.65 as builder

WORKDIR /usr/src/gymtracker
COPY . .
RUN cargo install --path gt-backend

FROM debian:buster-slim

RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/gt-backend /usr/local/bin/gt-backend

CMD ["gt-backend"]
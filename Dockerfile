FROM rust:1.65 as builder

RUN rustup target add wasm32-unknown-unknown && cargo install trunk

WORKDIR /usr/src/gymtracker
COPY . .
RUN cargo install --path gt-backend
RUN cd gt-frontend/ && ./build.sh --release

FROM debian:bullseye-slim

WORKDIR /app/gymtracker

COPY --from=builder /usr/local/cargo/bin/gt-backend /app/gymtracker/gt-backend
COPY --from=builder /usr/src/gymtracker/gt-frontend/dist /app/gymtracker/dist

CMD ["/app/gymtracker/gt-backend"]
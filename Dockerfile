FROM node:26-slim AS frontend-builder
WORKDIR /build/frontend
COPY frontend/package*.json ./
RUN npm install
COPY frontend/ .
RUN npm run build

FROM rust:1.95-slim AS backend-builder
WORKDIR /build/backend
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY backend/ .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=backend-builder /build/backend/target/release/backend ./battlefront

COPY --from=frontend-builder /build/frontend/dist ./dist

ENV RUST_LOG=info
ENV PORT=8080
EXPOSE 8080

CMD ["./battlefront"]

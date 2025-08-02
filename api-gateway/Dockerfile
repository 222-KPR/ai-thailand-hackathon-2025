# Stage 1: Build the application
FROM rust:1.82.0-slim-bullseye as builder

RUN rm -rf /var/lib/apt/lists/* && apt-get update && apt-get install -y libssl-dev pkg-config && apt-get clean

# Install build dependencies
WORKDIR /usr/src/app

# Create a dummy project to cache dependencies
RUN cargo init --bin .
COPY Cargo.toml Cargo.lock ./
COPY api-gateway/Cargo.toml ./api-gateway/Cargo.toml
COPY shared/Cargo.toml ./shared/Cargo.toml
COPY shared/src ./shared/src

COPY api-gateway/src ./api-gateway/src

# Build dependencies to cache them
RUN cargo build --release --package api-gateway --bin api-gateway --features=""
RUN rm -f target/release/deps/api_gateway*
RUN cargo build --release

# Stage 2: Create the final, minimal image
FROM debian:bullseye-slim as production

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/api-gateway /usr/local/bin/

# Set the command to run the application
CMD ["/usr/local/bin/api-gateway"]

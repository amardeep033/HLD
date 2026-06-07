# Stage 1: use the full Rust toolchain image to compile the application.
FROM rust:latest AS builder

# All following paths are relative to /app inside the temporary build container.
WORKDIR /app

# Copy dependency manifests first so Docker can cache dependency compilation
# separately from your application source code.
COPY Cargo.toml Cargo.lock ./

# Create a tiny placeholder app. This makes `cargo build` download and compile
# dependencies even before the real source code is copied.
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
# Remove the placeholder source so the real project can be copied next.
RUN rm -rf src

# Now copy the full project, including SQLx offline metadata and real source.
COPY . .

# SQLx can validate queries from checked-in metadata instead of needing a live DB
# connection during the image build.
ENV SQLX_OFFLINE=true

# Touching the file ensures Cargo sees a current source tree before the final
# release build of the real application binary.
RUN touch src/main.rs && cargo build --release

# Stage 2: start from a much smaller runtime image.
FROM debian:bookworm-slim

# Install only the shared libraries/certificates needed to run the compiled
# binary. The Rust compiler is not included in the final image.
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy just the compiled binary from the builder stage into the runtime image.
COPY --from=builder /app/target/release/transaction-service /usr/local/bin/transaction-service

# Documents the port the process listens on inside the container.
EXPOSE 8080

# Default process started when the container runs.
CMD ["transaction-service"]
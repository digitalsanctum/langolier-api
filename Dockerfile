# Stage 1: Build
ARG RUST_VERSION=1.71
FROM rust:${RUST_VERSION}-buster as builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the source code into the container
COPY . .

RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install --yes libcurl4 \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

# Build the release version of the microservice
RUN cargo install sqlx-cli
RUN cargo build --release --bin langolier-api


# Stage 2: Serve
FROM debian:buster-slim

# Install required libraries
RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install --yes libpsl5 libcurl4 libssl1.1 ca-certificates libnghttp2-14 librtmp1 libssh2-1 \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

# Create a new user to run the application
RUN useradd -ms /bin/bash appuser

# Set the working directory
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/langolier-api ./langolier-api
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /usr/lib/x86_64-linux-gnu/libcurl.so.4 /usr/lib/x86_64-linux-gnu/

# Set the ownership and permissions for the binary
RUN chown appuser:appuser ./langolier-api && \
    chmod 755 ./langolier-api

# Switch to the appuser
USER appuser

# Expose the port the app runs on
EXPOSE 3000
EXPOSE 8080

# Start the application
CMD ["./langolier-api"]

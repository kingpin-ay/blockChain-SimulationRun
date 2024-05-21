# Use the official Rust image as a base image
FROM rust:1.63-slim

# Set the working directory inside the container
WORKDIR /usr/src/myapp

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml  ./

# This build step is to cache dependencies
RUN cargo fetch

# Copy the source code to the container
COPY src ./src

# Build the application
RUN cargo build --release

# Run the application
CMD ["./target/release/myapp"]

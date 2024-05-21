# Use the official Rust image as a base image
FROM rust:1.78.0-slim-buster

# Set the working directory inside the container
WORKDIR /usr/src/myapp

# Copy the Cargo.toml and Cargo.lock files to the container
COPY ./  ./

# This build step is to cache dependencies
RUN cargo fetch


# Build the application
RUN cargo build --release

# Run the application
CMD ["./target/release/blockChain-SimulationRun"]

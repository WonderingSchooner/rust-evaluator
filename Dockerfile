# Use the official Rust image
FROM rust:latest

# Create the inbox directory
RUN mkdir -p /app/inbox

# Set the working directory
WORKDIR /app

# Copy the entire project to the container
COPY . .

# Build the project in release mode
RUN cargo build --release

# Set the entry point to run the Rust evaluator
CMD ["/app/target/release/rust-evaluator"]

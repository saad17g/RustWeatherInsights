# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to the working directory
COPY Cargo.toml Cargo.lock ./

# Copy the src directory to the working directory
COPY src ./src

# Copy the .env file to the working directory
COPY .env ./

# Build the Rust application
RUN cargo build --release

# Expose the port that your application listens on (if applicable)
# EXPOSE 8080

# Set the entry point for the container
ENTRYPOINT ["./target/release/rust_weather_insights"]
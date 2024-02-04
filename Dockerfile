FROM messense/rust-musl-cross:x86_64-musl as builder
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl
EXPOSE 3000
CMD ["./target/x86_64-unknown-linux-musl/release/checkers"]

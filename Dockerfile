FROM clux/muslrust

WORKDIR /app

# Build dependencies
COPY Cargo.toml dummy.rs /app/
RUN cargo build --lib --release

# Build app
COPY . /app
RUN cargo build --release

FROM alpine:edge
WORKDIR /usr/local/bin
COPY --from=0 /app/target/x86_64-unknown-linux-musl/release/rkt /usr/local/bin/

ENV ROCKET_ADDRESS 0.0.0.0
ENV ROCKET_PORT 8000
EXPOSE 8000
CMD ["/usr/local/bin/rkt"]

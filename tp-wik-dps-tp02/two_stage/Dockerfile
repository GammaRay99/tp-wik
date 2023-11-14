FROM rust as compiler

RUN useradd -r tp_app
WORKDIR /app
RUN chown -R tp_app:tp_app /app
RUN rustup target add x86_64-unknown-linux-musl

USER tp_app


RUN mkdir ./src && echo "fn main() {}" > ./src/main.rs
COPY --chown=tp_app ./app/Cargo.toml .
COPY --chown=tp_app ./app/Cargo.lock .
RUN cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -rf ./src
COPY --chown=tp_app ./app/src ./src
RUN touch -a -m ./src/main.rs
RUN cargo build --release --target=x86_64-unknown-linux-musl
CMD /app/target/x86_64-unknown-linux-musl/release/tp-wik-dps-tp01-bis


FROM scratch

COPY --from=compiler /app/target/x86_64-unknown-linux-musl/release/tp-wik-dps-tp01-bis /
CMD ["/tp-wik-dps-tp01-bis"]

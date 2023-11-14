FROM rust


RUN useradd -r tp_app
WORKDIR /app
RUN chown -R tp_app:tp_app /app

USER tp_app


RUN mkdir ./src && echo "fn main() {}" > ./src/main.rs
COPY --chown=tp_app ./app/Cargo.toml .
COPY --chown=tp_app ./app/Cargo.lock .
RUN cargo build --release

RUN rm -rf ./src
COPY --chown=tp_app ./app/src ./src
RUN touch -a -m ./src/main.rs
RUN cargo build --release
CMD /app/target/release/tp-wik-dps-tp01-bis
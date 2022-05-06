FROM rust:1

WORKDIR /usr/src/prisoner
COPY . .

RUN cargo install --path .

RUN chmod +x target/release/prisoner

CMD ["target/release"]

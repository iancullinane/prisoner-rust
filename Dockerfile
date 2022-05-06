FROM rust:1

WORKDIR /usr/src/prisoner
COPY . .

RUN cargo install --path .

CMD ["prisoner"]

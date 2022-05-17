FROM rust:1 as build

# create a new empty shell project
RUN USER=root cargo new --bin prisoner
WORKDIR /prisoner

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/prisoner*
RUN cargo build --release

# our final base
FROM rust:1

# copy the build artifact from the build stage
COPY --from=build /prisoner/target/release/prisoner .

# set the startup command to run your binary
ENTRYPOINT ["./prisoner"]
CMD ["--players", "3", "--rounds", "0"]

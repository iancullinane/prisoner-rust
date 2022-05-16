# Create the build container to compile the hello world program
FROM rust:1 as builder

ENV USER=prisoner
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /prisoner
USER ${USER}
COPY ./ .

RUN cargo build  --release
RUN chmod +x target/release/prisoner

CMD ["ls","-la", "target"]

# # Create the execution container by copying the compiled hello world to it and running it
# FROM scratch


# Import from builder.
# COPY --from=builder /etc/passwd /etc/passwd
# COPY --from=builder /etc/group /etc/group

# COPY --from=builder /prisoner/target/release/prisoner ./
# USER prisoner:prisoner
# CMD ["/prisoner"]

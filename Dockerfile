FROM messense/rust-musl-cross:x86_64-musl as builder
ENV SQLX_OFLFINE = true
WORKDIR /rust-api
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl


FROM scratch
COPY --from=builder /rust-api/target/x86_64-unknown-linux-musl/release/rust-contactbook-api /rust-api
ENTRYPOINT [ "/rust-api" ]
EXPOSE 3000
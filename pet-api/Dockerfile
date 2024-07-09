FROM messense/messense/rust-musl-cross:x86_64-musl as builder
ENV SQLX_OFFLINE=true
WORKDIR /pet-api
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /pet-api/target/x86_64-unknown-linux-musl/release/pet-api /pet-api
ENTRYPOINT [ "/pet-api" ]
EXPOSE 4000
FROM ghcr.io/rust-lang/rust:nightly as build-env
WORKDIR /app
COPY . /app
RUN cargo build --release --bin server

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/server /
EXPOSE 50051 50052
CMD ["./server"]
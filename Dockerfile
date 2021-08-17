FROM rust:alpine as build
RUN cargo search # This just caches the cargo index.
COPY . ./
RUN cargo build --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=build /target/x86_64-unknown-linux-musl/debug/whoami-bug /whoami-bug
ENTRYPOINT ["/whoami-bug"]

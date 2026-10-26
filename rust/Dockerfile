FROM rust:1-alpine AS build
RUN apk add --no-cache musl-dev ca-certificates
WORKDIR /src
COPY . .
RUN cargo build --release

FROM scratch
COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=build /src/target/release/ocap-rs /readme-fetch
ENTRYPOINT ["/readme-fetch", "idle"]

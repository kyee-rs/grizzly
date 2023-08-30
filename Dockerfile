FROM rust:1.72.0-alpine as builder

RUN apk add --no-cache musl-dev pkgconfig openssl-dev openssl perl make

WORKDIR /code
COPY . .

ENV CARGO_TARGET_DIR=/var/cache/cargo
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
RUN --mount=type=cache,target=${CARGO_TARGET_DIR} \
    cargo install --path .

FROM scratch
STOPSIGNAL SIGINT

COPY --from=builder /usr/local/cargo/bin/grizzly /grizzly

ENTRYPOINT ["/grizzly"]
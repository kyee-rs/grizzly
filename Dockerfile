# Copyright (C) 2023 Saputskyi Petro - All Rights Reserved
# You may use, distribute and modify this code under the
# terms of the CC-BY-SA-4.0 license.
#
# ----------------------------------------------------------------------------------------------------
# Commercial use - YES
# Distribution - YES
# Modification - YES
# Private use - YES
# ----------------------------------------------------------------------------------------------------
# Liability - NO
# Patent use - NO
# Trademark use - NO
# Warranty - NO
# ----------------------------------------------------------------------------------------------------
# A copy of the license and copyright notice must be included with the licensed material.
# Modifications must be released under the same license when distributing the licensed material.
# In some cases a similar or related license may be used.
# Changes made to the licensed material must be documented.
# ----------------------------------------------------------------------------------------------------
#
# You should have received a copy of the CC-BY-SA-4.0 license with
# this file. If not, please write to: hello@lowt.live, or visit: https://github.com/12subnet

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
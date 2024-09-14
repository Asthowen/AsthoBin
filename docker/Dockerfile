# Build image
# Necessary dependencies to build AsthoBin
FROM rust:alpine as build

LABEL version="0.0.2" maintainer="Asthowen<contact@asthowen.fr>"

WORKDIR "/asthobin"

RUN apk update && apk upgrade && \
    apk add --no-cache nodejs npm build-base cmake openssl-dev \
    && npm install -g pnpm

COPY . .

RUN cargo build --release --features https-support

# Release image
# Necessary dependencies to run AsthoBin
FROM alpine:latest

WORKDIR "/asthobin"

RUN ls
COPY --from=build /asthobin/target/release/asthobin .

ENTRYPOINT "./asthobin"
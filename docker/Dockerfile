# Build image
# Necessary dependencies to build AsthoBin
FROM rust:alpine as build

LABEL version="0.0.2" maintainer="Asthowen<contact@asthowen.fr>"

WORKDIR "/asthobin"

RUN apk update && apk upgrade && \
    apk add --no-cache perl perl-dev musl musl-dev alpine-sdk openssl-dev \
    pkgconfig cmake mysql-dev mariadb-dev gcompat \
    cmake gcc g++ make ncurses-dev

COPY . .

RUN cargo build --release

# Release image
# Necessary dependencies to run AsthoBin
FROM alpine:latest

WORKDIR "/asthobin"

RUN ls
COPY --from=build /asthobin/target/release/asthobin .

ENTRYPOINT "./asthobin"
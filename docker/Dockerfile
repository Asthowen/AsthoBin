# Build image
# Necessary dependencies to build AsthoBin
FROM rust:latest as build

LABEL version="0.0.2" maintainer="Asthowen<contact@asthowen.fr>"

RUN apt update && apt install -y build-essential default-libmysqlclient-dev curl
RUN curl -fsSL https://deb.nodesource.com/setup_current.x | bash - && apt install -y nodejs && npm install -g pnpm

WORKDIR "/asthobin"

COPY . .
RUN cargo build --release

# Release image
# Necessary dependencies to run AsthoBin
FROM debian:latest

RUN apt update && apt install -y default-libmysqlclient-dev

WORKDIR "/asthobin"

COPY --from=build /asthobin/target/release/asthobin .

ENTRYPOINT "./asthobin"
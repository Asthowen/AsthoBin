# Build image
# Necessary dependencies to build AsthoBin
FROM rust:alpine AS build

LABEL version="0.0.2" maintainer="Asthowen<contact@asthowen.fr>"

WORKDIR "/asthobin"

RUN apk update && apk upgrade && \
    apk add --no-cache nodejs npm build-base cmake \
    && npm install -g pnpm

COPY . .

RUN cargo build --release

# Release image
# Necessary dependencies to run AsthoBin
FROM alpine:latest

WORKDIR "/asthobin"

COPY --from=build /asthobin/target/release/asthobin .

CMD ["./asthobin"]
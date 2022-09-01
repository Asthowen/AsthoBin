# Build image
# Necessary dependencies to build AsthoBin
FROM rust:alpine as build

LABEL version="0.0.2" maintainer="Asthowen<contact@asthowen.fr>"

RUN apk add --no-cache --update openssl-dev perl alpine-sdk libffi-dev tzdata npm gcc musl-dev mariadb-connector-c-dev

WORKDIR "/asthobin"

COPY . .
RUN cargo build --release && npm i && npm run prod

# Release image
# Necessary dependencies to run AsthoBin
FROM alpine:latest

RUN apk add --no-cache --update tzdata
ENV TZ = Europe/Paris
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone && mkdir /asthobin

WORKDIR "/asthobin"

COPY --from=build /asthobin/target/release/asthobin .
COPY --from=build /asthobin/static ./static

ENTRYPOINT "./asthobin"
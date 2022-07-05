FROM rust:latest

LABEL version="0.0.1" maintainer="Asthowen<contact@asthowen.fr>"

RUN apt update && apt install wget nodejs npm -y
WORKDIR "/asthobin"
COPY ./ /asthobin

RUN wget "https://github.com/Asthowen/AsthoBin/releases/download/0.0.1/asthobin-0.0.1-x86_64-unknown-linux-musl" && chmod +x ./asthobin-0.0.1-x86_64-unknown-linux-musl && npm i && npm run prod

ENTRYPOINT ./asthobin-0.0.1-x86_64-unknown-linux-musl
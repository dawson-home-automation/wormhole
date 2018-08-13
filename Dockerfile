FROM rust

MAINTAINER Dawson Freitas Israel

RUN mkdir /app
WORKDIR /app
COPY ./ /app
RUN cargo build --release

CMD ["./target/release/wormhole"]

FROM rust:1.70.0-slim-buster as build

WORKDIR /app

COPY Cargo.toml Cargo.lock /app/
COPY src/ /app/src

RUN apt update -y && apt upgrade -y && apt install libsqlite3-dev -y

RUN cargo build -r

FROM debian:buster-slim as run

WORKDIR /app

COPY --from=build /app/target/release/redirect ./
COPY --from=build /usr/lib/x86_64-linux-gnu/libsqlite3.so.0 /usr/lib/x86_64-linux-gnu/

ENV DB_TYPE=SQLITE3
ENV SQLITE_PATH=./redirect.db

CMD [ "./redirect" ]
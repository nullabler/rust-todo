FROM rust

WORKDIR backend

COPY ./backend .
COPY ./docker/backend/entrypoint .

RUN apt update && apt install -y default-mysql-client
RUN cargo install cargo-watch

ENTRYPOINT ["/bin/sh", "-c", "/backend/entrypoint"]

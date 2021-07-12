FROM rust

WORKDIR backend
COPY ./backend .

RUN apt update && apt install -y default-mysql-client
RUN cargo install cargo-watch

CMD ["cargo", "watch", "--exec", "run"]

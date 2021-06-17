FROM rust

WORKDIR backend
COPY ./backend .

RUN cargo install cargo-watch

CMD ["cargo", "watch", "--exec", "run"]

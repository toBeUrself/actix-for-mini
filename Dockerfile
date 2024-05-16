FROM rust:latest

WORKDIR /Users/timliu/rust/actix-for-mini

COPY . .

RUN cargo build

EXPOSE 3000

CMD [".runapp.sh"]

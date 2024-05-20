FROM rust:latest

WORKDIR /app

RUN pwd

COPY . .

RUN ls -a

RUN cargo build --release

EXPOSE 3000

RUN ls -a

CMD ["./startapp.sh"]

FROM rust:latest AS builder

WORKDIR /app

# RUN apt-get -y update && \
#     apt-get -y upgrade && \
#     apt-get -y install software-properties-common curl git vim tree

# # SYSTEM DEPENDENCIES
# RUN apt-get install -y \
#     build-essential \
#     llvm-dev \
#     libclang-dev \
#     clang \
#     openssl \
#     pkg-config \
#     libssl-dev \
#     xz-utils


COPY . .

ENV MYSQL_HOST=mysql-test

RUN cargo build --release

FROM ubuntu:latest AS runtime

RUN apt-get -y update && \
    apt-get -y upgrade && \
    apt-get -y install build-essential software-properties-common curl git vim tree

COPY --from=builder /app/target/release/actix-for-mini ./exe
COPY --from=builder /app/startapp.sh ./startapp.sh

EXPOSE 3000

CMD ["./startapp.sh"]

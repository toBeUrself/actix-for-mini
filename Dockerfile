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

# 通过docker网络别名访问数据库
ENV MYSQL_HOST=mysql-test

RUN cargo build --release

FROM ubuntu:latest AS runtime

RUN apt-get -y update && \
    apt-get -y upgrade && \
    apt-get -y install build-essential software-properties-common curl git vim tree

COPY --from=builder /app/target/release/actix-for-mini ./exe
COPY --from=builder /app/startapp.sh ./startapp.sh

# 创建临时文件目录并映射到宿主机
RUN mkdir -p /tmp/my-data/mini-images && \
    chown -R root:root /tmp/my-data/mini-images  && \
    chmod -R 755 /tmp/my-data/mini-images
# VOLUME /tmp/my-data/mini-images

EXPOSE 3000

CMD ["./startapp.sh"]

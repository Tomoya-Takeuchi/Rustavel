FROM --platform=linux/amd64 rust:1.65-buster

RUN apt-get update && apt-get install -y mariadb-client

WORKDIR /home/rustavel
COPY . .

ENTRYPOINT ["bash"]

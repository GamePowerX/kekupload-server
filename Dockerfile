FROM rustlang/rust:nightly as builder
WORKDIR /app
COPY . .

RUN chmod +x configure.sh

RUN cargo install diesel_cli
RUN cargo build --release
RUN cp ./target/release/uploadserver .
RUN rm -rf target

FROM ubuntu:rolling
EXPOSE 6942
ENV DEBIAN_FRONTEND=noninteractive
COPY --from=builder /usr/local/cargo/bin/diesel /usr/bin
COPY --from=builder /app /app
RUN apt update 
RUN apt install -y libpq-dev
RUN apt install -y libmysqlclient-dev
RUN apt install -y libmariadb-dev
RUN apt install -y libsqlite3-dev
RUN rm -rf /var/lib/apt/lists/*
WORKDIR /app/
CMD sh configure.sh

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
COPY --from=builder /app /app
RUN apt update && apt install -y libpq-dev
RUN rm -rf /var/lib/apt/lists/*
WORKDIR /mount
CMD ["../app/configure.sh"]
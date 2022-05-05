FROM rustlang/rust:nightly as builder
WORKDIR /app
COPY . .

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
RUN apt install -y git

# Install NodeJS
ENV NODE_VERSION=16.13.0
RUN apt install -y curl
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
ENV NVM_DIR=/root/.nvm
RUN . "$NVM_DIR/nvm.sh" && nvm install ${NODE_VERSION}
RUN . "$NVM_DIR/nvm.sh" && nvm use v${NODE_VERSION}
RUN . "$NVM_DIR/nvm.sh" && nvm alias default v${NODE_VERSION}
ENV PATH="/root/.nvm/versions/node/v${NODE_VERSION}/bin/:${PATH}"
RUN node --version
RUN npm --version

RUN rm -rf /var/lib/apt/lists/*
WORKDIR /app/
CMD ./uploadserver

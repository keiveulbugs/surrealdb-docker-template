FROM rust:latest

RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
        libclang-dev

WORKDIR /usr/src/surrealdb-docker-template


# Copy the source code
COPY . .

RUN cargo install --features "database" --path .

CMD ["surrealdb-docker-template"]
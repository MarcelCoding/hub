FROM rust:latest AS rust

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# Create appuser
ENV USER=hub
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

RUN cargo new --bin hub

WORKDIR /hub

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --target x86_64-unknown-linux-musl --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/x86_64-unknown-linux-musl/release/deps/hub*
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM node:16-alpine AS web

WORKDIR /src

COPY web/package.json web/package-lock.json ./

RUN npm install --production

COPY web/ .

RUN npm run build

FROM alpine

ENV HUB_ADDR=0.0.0.0:3030
ENV HUB_WEB_ROOT=/web
ENV HUB_DATA_DIR=/data

COPY --from=rust /etc/passwd /etc/passwd
COPY --from=rust /etc/group /etc/group

WORKDIR /data

COPY --from=rust /hub/target/x86_64-unknown-linux-musl/release/hub /hub
COPY --from=web /src/dist /web

RUN chmod -R 555 /web
RUN chmod -R 555 /hub
RUN chown -R hub:hub /data

USER hub:hub

CMD ["/hub"]

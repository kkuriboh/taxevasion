FROM rust:slim as builder
WORKDIR /opt
COPY . .

ARG BOT_TOKEN
ENV BOT_TOKEN=$BOT_TOKEN
ARG REPLY_GIF_URL
ENV REPLY_GIF_URL=$REPLY_GIF_URL

RUN cargo build --release

FROM debian:bullseye-slim as runner
WORKDIR /opt
COPY --from=builder /opt/target/release/taxevasion .
ENTRYPOINT ["./taxevasion"]

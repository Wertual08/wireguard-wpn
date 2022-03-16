FROM rust:alpine as builder
WORKDIR /app
COPY . .
RUN cargo install --path .


FROM alpine
RUN apk add --no-cache --update \
    wireguard-tools \
    libc6-compat 
 
COPY --from=builder /usr/local/cargo/bin/wireguard-manager /wireguard-manager

EXPOSE 51820/udp

CMD wg-quick up /etc/wireguard/wg0.conf ; /wireguard-manager
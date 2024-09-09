FROM rust:alpine3.20 as BUILDER
WORKDIR /home
RUN apk add --no-cache musl-dev build-base
COPY . .
RUN cargo build --release


FROM alpine:latest as RUNTIME
WORKDIR /home
RUN apk add --no-cache libgcc libstdc++
COPY --from=builder /home/target/release/localhost_pastebin /home/localhost_pastebin 
COPY ./index.html ./index.html
EXPOSE 9900
CMD ["/home/localhost_pastebin"]

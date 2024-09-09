FROM rust:alpine3.20 as BUILDER
WORKDIR /home
COPY ./src .
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release




FROM alpine:latest as RUNTIME
WORKDIR /home
RUN apk add --no-cache libgcc libstdc++
COPY --from=builder /home/target/localhost_pastebin ./ 
COPY ./index.html ./index.html
CMD ["target/localhost_pastebin"]

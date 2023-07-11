FROM rust:1.69 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /app
COPY . .

RUN cargo build --release 
# FROM gcr.io/distroless/cc-debian10
FROM rust:1.69 

COPY --from=build /app/target/release/ch18 /usr/local/bin/ch18

EXPOSE 3000
ENTRYPOINT ["ch18"]

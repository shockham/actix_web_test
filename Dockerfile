FROM alpine:edge as builder
RUN apk --no-cache add rust cargo
WORKDIR /app/
COPY . .
RUN cargo b --release

FROM alpine:latest  
RUN apk --no-cache add ca-certificates libgcc libstdc++ eudev
WORKDIR /app/
COPY --from=builder /app/target/release/actix_web_test .
EXPOSE 80
CMD ["./actix_web_test"]

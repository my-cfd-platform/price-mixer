FROM ubuntu:22.04
COPY ./target/release/price-mixer ./target/release/price-mixer

ENTRYPOINT ["./target/release/price-mixer"]
FROM rust:1.59

ENV RUST_BACKTRACE=full

COPY ./ ./

RUN rustup default nightly
RUN cargo install --path  .

CMD ["coinloan_test"]

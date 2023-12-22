
# # Use prebuilt builder image
# FROM rust-prebuilt as builder
FROM rust:latest as builder
WORKDIR /usr/src
# # New cargo project and copy Rust dependencies (and store as a separate Docker layer)
# # NOTE: must call `cargo vendor` first and add `vendor` folder to git
RUN USER=root cargo new wanyeel
WORKDIR /usr/src/wanyeel

COPY .cargo .cargo
COPY vendor vendor
COPY Cargo.toml Cargo.lock ./
COPY cut_optimizer_1d cut_optimizer_1d
COPY cut_optimizer_2d cut_optimizer_2d
COPY src src
COPY wx wx
COPY wy_common wy_common
COPY wy_domain wy_domain
COPY application.yaml application.yaml

RUN cargo install --path . --color always

RUN cp ./target/release/wanyeel ./

RUN rm -rf .cargo vendor cut-optimizer-1d  cut-optimizer-2d src wx wy_common wy_domain target

EXPOSE 5001

ENTRYPOINT ["./wanyeel"]
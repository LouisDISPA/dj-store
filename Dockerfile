FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM node:18 as ui-builder

WORKDIR /app 

COPY ui/package.json /app
COPY ui/yarn.lock /app

RUN yarn install 
COPY ui/ /app/

RUN yarn build

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY --from=ui-builder /app/build ./ui/build

COPY src ./src
COPY entity ./entity
COPY migration ./migration
COPY Cargo* ./

RUN cargo build --release

# We do not need the Rust toolchain to run the binary!
FROM debian:bullseye AS runtime
RUN apt-get update && apt-get install -y libcurl4

WORKDIR app
ENV DATABASE_URL=sqlite://db.sqlite
RUN touch db.sqlite

COPY --from=builder /app/target/release/dj-store /usr/local/bin


ENTRYPOINT ["/usr/local/bin/dj-store"]
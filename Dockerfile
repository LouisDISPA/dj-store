# --- Build the cargo chef utility container ---
# This container is used to generate a cargo chef recipe
# which is used to cache dependencies for the rust backend

FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
RUN apt-get update
RUN apt-get install -y musl-tools upx
WORKDIR /app

# --- Generate the cargo chef recipe ---

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# --- Build the UI ---

FROM node:18 as ui-builder
WORKDIR /app 

# Install dependencies
COPY ui/package.json /app
COPY ui/yarn.lock /app
RUN yarn install 

COPY ui/ /app/
RUN yarn build

# --- Build the backend ---

FROM chef AS builder 

# Install musl target
# TODO: Use this docker compose also for the aarch64 build
ENV CARGO_BUILD_TARGET="x86_64-unknown-linux-musl"
RUN rustup target add ${CARGO_BUILD_TARGET}

# Cache dependencies with cargo chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Copy the ui build
COPY --from=ui-builder /app/build ./ui/build

COPY src ./src
COPY entity ./entity
COPY migration ./migration
COPY deezer-rs ./deezer-rs
COPY Cargo* ./

RUN cargo build --release
RUN mv /app/target/${CARGO_BUILD_TARGET}/release/dj-store /dj-store
RUN upx --lzma --best /dj-store

# --- Build the final image ---
# This image is from scratch and only contains the binary

FROM scratch AS runtime
COPY --from=builder /dj-store /

ENV DATABASE_URL=sqlite://db.sqlite

ENTRYPOINT ["/dj-store"]
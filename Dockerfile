# --- Build the cargo chef utility container ---
# This container is used to generate a cargo chef recipe
# which is used to cache dependencies for the rust backend

FROM rust:alpine AS chef
RUN apk add --no-cache musl-dev
RUN cargo install cargo-chef
RUN apk add --no-cache upx
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

# Cache dependencies with cargo chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Copy the ui build
COPY --from=ui-builder /app/build ./ui/build

COPY src ./src
COPY entity ./entity
COPY migration ./migration
COPY deezer-rs ./deezer-rs
COPY api-macro ./api-macro
COPY Cargo* ./

RUN cargo build --release
RUN mv /app/target/release/dj-store /dj-store
RUN upx --lzma --best /dj-store
RUN touch /db.sqlite

# --- Build the final image ---
# This image is from scratch and only contains the binary

FROM scratch AS runtime
COPY --from=builder /dj-store /
COPY --from=builder /db.sqlite /

ENV DATABASE_URL=sqlite://db.sqlite
ENV RUST_LOG=warn,dj_store=info

EXPOSE 3000
ENTRYPOINT ["/dj-store" "0.0.0.0:3000"]

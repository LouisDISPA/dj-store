# --- Build the cargo chef utility container ---
# This container is used to generate a cargo chef recipe
# which is used to cache dependencies for the rust backend

FROM clux/muslrust:stable AS chef
USER root
RUN cargo install cargo-chef
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
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

# Copy the ui build
COPY --from=ui-builder /app/build ./ui/build

COPY src ./src
COPY entity ./entity
COPY migration ./migration
COPY Cargo* ./

RUN cargo build --release --target x86_64-unknown-linux-musl

# --- Build the final image ---
# This image is from scratch and only contains the binary
# It also needs ssl certificates for curl to work

FROM scratch AS runtime

# Copy the ssl certificates from the builder image and the binary
COPY --from=builder /etc/ssl/certs /etc/ssl/certs
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/dj-store /

ENV DATABASE_URL=sqlite://db.sqlite

ENTRYPOINT ["/dj-store"]
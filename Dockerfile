## Build react files
FROM node:14.2 as front
RUN mkdir -p /web
WORKDIR /web

# cache dependencies
COPY ./web/package.json ./web/yarn.lock ./
RUN yarn

# copy source
COPY ./web/ ./

RUN yarn test
RUN yarn build

## Compile actix-web server
FROM rust:1.43 as back

# create a new empty shell project
RUN USER=root cargo new --bin picatch_source
WORKDIR /picatch_source

# since both lib and bin provided in Cargo.toml, need to create lib.rs to build
RUN touch ./src/lib.rs

# copy over manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy source tree
COPY ./src ./src

# copy test files
COPY ./tests ./tests

# copy built static react files
RUN mkdir -p ./web/build
COPY --from=front /web/build ./web/build

# build for release, remove dummy compiled files **including libpicatch**
RUN rm ./target/release/deps/*picatch*

RUN cargo test --release
RUN cargo build --release

## Final base image with only the picatch binary
FROM debian:buster-slim
COPY --from=back /picatch_source/target/release/picatch /picatch

# Default dirs
ENV PICATCH_ORIGINAL_PHOTOS_DIR="/photos"
ENV PICATCH_RESIZED_PHOTOS_DIR="/photos_resized"

# Dir for external photos
RUN mkdir -p /photos
RUN mkdir -p /photos_resized
EXPOSE 8080
ENTRYPOINT ["/picatch"]

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

# build for release, remove dummy compiled files **including libpicatch**
RUN rm ./target/release/deps/*picatch*
RUN cargo build --release


## Build react files
FROM node:14.2 as front
RUN mkdir -p /web
WORKDIR /web

# cache dependencies
COPY ./web/package.json ./web/yarn.lock ./
RUN yarn

# copy source
COPY ./web/ ./
RUN yarn build

## Final base image
FROM debian:buster-slim
COPY --from=back /picatch_source/target/release/picatch_bin /picatch
COPY --from=front /web/build /public

# Default dirs
ENV PICATCH_PHOTOS_DIR="/photos"
ENV PICATCH_PUBLIC_DIR="/public"

# Dir for external photos
RUN mkdir -p /photos
EXPOSE 8080
ENTRYPOINT ["/picatch"]

# Compile actix-web server
FROM rust:1.43 as back
RUN mkdir -p /sources
WORKDIR /sources
COPY . /sources
RUN cargo build --release

# Build react files
FROM node:14.2 as front
RUN mkdir -p /web
WORKDIR /web
COPY ./web/ /web
RUN yarn && yarn build

FROM debian:buster-slim
COPY --from=back /sources/target/release/dphoto_bin /dphoto
COPY --from=front /web/build /public

# Default dirs
ENV PHOTOS_DIR="/photos"
ENV PUBLIC_DIR="/public"

# Dir for external photos
RUN mkdir -p /photos
EXPOSE 8080
ENTRYPOINT ["/dphoto"]

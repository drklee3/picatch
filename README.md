# picatch

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/drklee3/picatch/Docker?style=flat-square)

![picatch logo](./logo.png)

Minimal photo gallery based on directory structure without a database.

<details>
  <summary>(picatch: pic + catch)</summary>
  Why catch? It... catches pictures in a folder? uhh I don't really know
</details>

## Running

Docker images are provided via GitHub Packages.  Binary downloads coming soon.

First [authenticate with GitHub Packages] by creating a personal access token
with at least the `read:packages` scope.

To run picatch with `docker run`

```bash
docker run \
    -p 8080:8080 \
    -v /path/to/your/photos:/photos \
    docker.pkg.github.com/drklee3/picatch/picatch:latest
```

Alternatively with `docker-compose`

```yml
version: '3'
services:
  picatch:
    image: docker.pkg.github.com/drklee3/picatch/picatch:latest
    ports:
      - "8080:8080"
    volumes:
      - /path/to/your/photos:/photos
```

## Compiling from source

### Requirements

#### Backend

* Rust version 1.40.0+

#### Frontend

* Node.js
* Yarn

## Building

### Frontend

```
cd web

# Build frontend files
yarn && yarn build
```

### Backend

```bash
# Install Rust -- https://www.rust-lang.org/tools/install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone this repository
git clone https://github.com/drklee3/picatch.git

cd picatch

# Build for release
cargo build --release
```

When compiling with the release profile, static files are embedded in the binary.
This means you need to build the frontend files **before** compiling picatch so
the files are correctly embedded.

## Running

```bash
./target/release/picatch_bin
```

## Configuration

When running the binary directly, you can specify which directories files are
served from along with the interface and port via environment variables.  If you
are running picatch via Docker, you should set the photos directory with a bind
mount as shown above.

| Environment Variable | Default value |
| -------------------- | ------------- |
| `PICATCH_PHOTOS_DIR` | `./photos`    |
| `PICATCH_INTERFACE`  | `0.0.0.0`     |
| `PICATCH_PORT`       | `8080`        |

## backend

* [Actix web](https://github.com/actix/actix-web) server
* image resizer w/ [image](https://github.com/image-rs/image)

## client

* todo

## TODO / Features

* [ ] indexer api
  * [ ] cache
  * [ ] image resizing
  * [ ] exif data
  * [ ] tags?
* [ ] Uh front end
  * [ ] Home
  * [ ] Albums
  * [ ] Images

[authenticate with GitHub Packages]: https://help.github.com/en/packages/using-github-packages-with-your-projects-ecosystem/configuring-docker-for-use-with-github-packages#authenticating-to-github-packages

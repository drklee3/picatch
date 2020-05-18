# picatch <!-- omit in toc -->

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/drklee3/picatch/Docker?style=flat-square)

![picatch logo](./logo.png)

Minimal photo gallery based on directory structure without a database.

<details>
  <summary>(picatch: pic + catch)</summary>
  Why catch? It... catches pictures in a folder? uhh I don't really know
</details>

## Table of Contents <!-- omit in toc -->

- [Running](#running)
  - [via Docker](#via-docker)
  - [via binary](#via-binary)
- [Configuration](#configuration)
- [Compiling from source](#compiling-from-source)
  - [Requirements](#requirements)
    - [Backend](#backend)
    - [Frontend](#frontend)
  - [Building](#building)
    - [Frontend](#frontend-1)
    - [Backend](#backend-1)
  - [Running](#running-1)

## Running

### via Docker

Docker images are provided via GitHub Packages.

First [authenticate with GitHub Packages][github-package-auth] by creating a
personal access token with at least the `read:packages` scope.

To run picatch with `docker run`

```bash
docker run \
    -p 8080:8080 \
    -v /path/to/your/photos:/photos \
    docker.pkg.github.com/drklee3/picatch/picatch:latest
```

Alternatively with `docker-compose`

```yml
version: "3"
services:
  picatch:
    image: docker.pkg.github.com/drklee3/picatch/picatch:latest
    ports:
      - "8080:8080"
    volumes:
      - /path/to/your/photos:/photos
```

### via binary

If you don't want to use docker, you can download the latest pre-built binary
from the [GitHub Actions artfacts][gh-workflow-ci].

Then simply run the `picatch` executable.

## Configuration

When running the binary directly, you can specify which directories files are
served from along with the interface and port via environment variables. If you
are running picatch via Docker, you should set the photos directory with a bind
mount as shown above.

| Environment Variable | Default value |
| -------------------- | ------------- |
| `PICATCH_PHOTOS_DIR` | `./photos`    |
| `PICATCH_INTERFACE`  | `0.0.0.0`     |
| `PICATCH_PORT`       | `8080`        |

## Compiling from source

### Requirements

#### Backend

- [Rust](https://www.rust-lang.org/tools/install) version 1.40.0+

#### Frontend

- Node.js
- Yarn

### Building

#### Frontend

```
cd web

# Build frontend files
yarn && yarn build
```

If you want to only build the backend yourself (if you don't have node.js
installed and don't want to install it or something), you can also download the
built frontend files specifically from the [GitHub Actions artifacts][gh-workflow-ci].

Extract the frontend files to `./web/build` then build the backend in the next step.

```bash
# In project root
mkdir ./web/build
unzip picatch-xxx-frontend.zip -d ./web/build
```

#### Backend

```bash
# Build for release
cargo build --release
```

**Note:** When compiling with the release profile, static files are embedded in the binary.
This means you need to build the frontend files **before** compiling picatch so
the files are correctly embedded.

### Running

```bash
./target/release/picatch_bin
```

## Info <!-- omit in toc -->

### backend <!-- omit in toc -->

- [Actix web][actix-web] server
- image resizer w/ [image]

### client <!-- omit in toc -->

- todo

### TODO / Features <!-- omit in toc -->

- [ ] indexer api
  - [ ] cache
  - [ ] image resizing
  - [ ] exif data
  - [ ] tags?
- [ ] Uh front end
  - [ ] Home
  - [ ] Albums
  - [ ] Images

[actix-web]: https://github.com/actix/actix-web
[gh-workflow-ci]: https://github.com/drklee3/picatch/actions?query=workflow%3ACI
[github-package-auth]: https://help.github.com/en/packages/using-github-packages-with-your-projects-ecosystem/configuring-docker-for-use-with-github-packages#authenticating-to-github-packages
[image]: https://github.com/image-rs/image

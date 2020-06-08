# picatch <!-- omit in toc -->

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/drklee3/picatch/Docker?style=flat-square)

![picatch logo](./logo.png)

Minimal photo gallery based on directory structure without a database. Utilizes
[Actix web][actix-web].

<details>
  <summary>(picatch: pic + catch)</summary>
  Why catch? It... catches pictures in a folder? uhh I don't really know
</details>

## Table of Contents <!-- omit in toc -->

- [Running](#running)
  - [via Docker](#via-docker)
  - [via binary](#via-binary)
- [Configuration](#configuration)
  - [Config File](#config-file)
  - [Environment Variables](#environment-variables)
    - [Application Options](#application-options)
    - [Public Options](#public-options)
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

Most configuration options can be set either by environment variables or a
config file.

**Note:** If you are running picatch via Docker, you should set the directory
options with bind mounts as shown above. Interface and port should be configured
through Docker options as well.

### Config File

The config location can be set via the following environment variable. By
default, picatch will look for a config relative to the current working
directory.

| Environment Variable | Default Value | Example         |
| -------------------- | ------------- | --------------- |
| `PICATCH_CONFIG`     | `./picatch`   | `/picatch.toml` |

The default config options are shown below.

```toml
# Directory where you can add full resolution photos.
original_photos_dir = "./photos"
# Resized photos from original_photos_dir. You should not modify files in here.
resized_photos_dir = "./photos_resized"

# Web server interface
interface = "0.0.0.0"
# Web server port
port = 8080

[public]
# Name of website shown on frontend and title
site_name = "picatch"

# Custom links in navbar
[[public.links]]
text = "Picatch"
url = "https://github.com/drklee3/picatch"

[[public.links]]
text = "GitHub"
url = "https://github.com/drklee3/"
```

### Environment Variables

Log level can _only_ be set from the `PICATCH_lOG` environment variable as
shown below. All other configuration options can be set via the config file
and/or environment variables. Environment variables will override config file
settings.

| Environment Variable | Default Value | Available options                                                   |
| -------------------- | ------------- | ------------------------------------------------------------------- |
| `PICATCH_LOG`        | `INFO`        | `OFF`, `ERROR`, `WARN`, `INFO`, `DEBUG`, `TRACE` (case insensitive) |

#### Application Options

| Environment Variable         | Default Value       |
| ---------------------------- | ------------------- |
| `PICATCH_PHOTOS_DIR`         | `./photos`          |
| `PICATCH_RESIZED_PHOTOS_DIR` | `./photos_resized/` |
| `PICATCH_INTERFACE`          | `0.0.0.0`           |
| `PICATCH_PORT`               | `8080`              |

#### Public Options

Additional configuration options can be set to be displayed on the frontend such
as the site name and displayed links.

| Environment Variable           | Default Value | Example |
| ------------------------------ | ------------- | ------- |
| `PICATCH_PUBLIC.SITE_NAME`     | `picatch`     |         |
| `PICATCH_PUBLIC.LINKS[i].TEXT` |               |         |
| `PICATCH_PUBLIC.LINKS[i].URL`  |               |         |

Since there can be multiple links, `PICATCH_PUBLIC.LINKS` can be treated as an
array. Replace `i` with the corresponding link index.

Example usage shown below. This provides the _same_ results as the example toml
configuration file shown [above](#config-file).

```bash
env PICATCH_LOG=debug \
    PICATCH_PUBLIC.LINKS[0].TEXT=Picatch \
    PICATCH_PUBLIC.LINKS[0].URL=https://github.com/drklee3/picatch \
    PICATCH_PUBLIC.LINKS[1].TEXT=GitHub \
    PICATCH_PUBLIC.LINKS[1].URL=https://github.com/drklee3/ \
    picatch
```

## Compiling from source

### Requirements

#### Backend

- [Rust](https://www.rust-lang.org/tools/install) version 1.40.0+

#### Frontend

- Node.js
- Yarn

### Building

#### Frontend

```bash
cd web

# Build frontend files
yarn && yarn build
```

If you want to only build the backend yourself (if you don't have Node.js
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

[actix-web]: https://github.com/actix/actix-web
[gh-workflow-ci]: https://github.com/drklee3/picatch/actions?query=workflow%3ACI
[github-package-auth]: https://help.github.com/en/packages/using-github-packages-with-your-projects-ecosystem/configuring-docker-for-use-with-github-packages#authenticating-to-github-packages
[image]: https://github.com/image-rs/image

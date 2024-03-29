# picatch <!-- omit in toc -->

[![GitHub Workflow Status][github-workflow-docker-img]][github-workflow-docker]
[![Docker Image Size (tag)][docker-hub-img]][docker-hub]
[![Master Demo][demo-master-img]][demo-master]

![picatch logo](./logo.png)

⚠️ **Warning:** picatch is still in under early development. Major features and
proper ui/ux are a work in progress. Major breaking changes are to be expected.

Minimal photo gallery based on directory structure without a database. Utilizes
[Actix web][actix-web].

<details>
  <summary>(picatch: pic + catch)</summary>
  Why catch? It... catches pictures in a folder? uhh I don't really know
</details>

## Demo <!-- omit in toc -->

[stable][demo-stable] (Last version tagged)

[master][demo-master] (Built from master branch)

## Table of Contents <!-- omit in toc -->

- [Running](#running)
  - [via Docker](#via-docker)
  - [via binary](#via-binary)
- [Configuration](#configuration)
  - [Config File](#config-file)
  - [Environment Variables](#environment-variables)
    - [Application Options](#application-options)
    - [Public Options](#public-options)
    - [Example](#example)
- [Compiling From Source](#compiling-from-source)
  - [Requirements](#requirements)
    - [Backend](#backend)
    - [Frontend](#frontend)
  - [Building](#building)
    - [Frontend](#frontend-1)
    - [Backend](#backend-1)
  - [Running](#running-1)
- [License](#license)

## Running

### via Docker

Docker images are provided via both [Docker Hub][docker-hub] and [GitHub
Packages][github-packages].

**Note:** If you are downloading from GitHub Packages, you will need to first
[authenticate with GitHub Packages][github-package-auth] by creating a personal
access token with at least the `read:packages` scope.

To download from GitHub Packages instead of Docker Hub, replace the following
instances of `drklee3/picatch` with
`docker.pkg.github.com/drklee3/picatch/picatch`

The `latest` and `stable` tags provide the most recent version tagged image. To
run the most recent image built from the git master branch, use the Docker image
tag `master`.

To run picatch with `docker run`

```bash
docker run \
    -p 8080:8080 \
    -v /path/to/your/photos:/photos \
    drklee3/picatch
```

Alternatively with `docker-compose`

```yml
version: "3"
services:
    picatch:
        image: drklee3/picatch
        ports:
            - "8080:8080"
        volumes:
            - /path/to/your/photos:/photos
```

### via binary

If you don't want to use Docker, you can download the latest pre-built binary
from the [GitHub Actions artfacts][github-workflow-ci].

Then simply run the `picatch` executable.

## Configuration

Most configuration options can be set either by environment variables or a
config file. Configuration options can be set via the config file and/or
environment variables. The options that can **only** be set via environment
variables are `PICATCH_CONFIG` and `PICATCH_LOG`.

Environment variables will **override** config file settings.

**Note:** If you are running picatch via Docker, you should set the directory
options with bind mounts as shown above. Interface and port should be configured
through Docker options as well. The only options you should be changing via
config file or environment variables are the public options.

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

**Note:** Environment variables can also be set in an `.env` file. However, it
should be easier to set configuration options via a toml file as shown above.
Environment variables should be used for a smaller number of options or to
override the config file.

Log level can only be set from the `PICATCH_lOG` environment variable as shown
below.

| Environment Variable | Default Value | Available options                                                       |
| -------------------- | ------------- | ----------------------------------------------------------------------- |
| `PICATCH_LOG`        | `INFO`        | `OFF`, `ERROR`, `WARN`, `INFO`, `DEBUG`, `TRACE`<br/>(case insensitive) |

#### Application Options

| Environment Variable          | Default Value      |
| ----------------------------- | ------------------ |
| `PICATCH_ORIGINAL_PHOTOS_DIR` | `./photos`         |
| `PICATCH_RESIZED_PHOTOS_DIR`  | `./photos_resized` |
| `PICATCH_INTERFACE`           | `0.0.0.0`          |
| `PICATCH_PORT`                | `8080`             |

#### Public Options

Additional configuration options can be set to be displayed on the frontend such
as the site name and displayed links.

| Environment Variable           | Default Value |
| ------------------------------ | ------------- |
| `PICATCH_PUBLIC.SITE_NAME`     | `picatch`     |
| `PICATCH_PUBLIC.LINKS[i].TEXT` |               |
| `PICATCH_PUBLIC.LINKS[i].URL`  |               |

Since there can be multiple links, `PICATCH_PUBLIC.LINKS` can be treated as an
array. Replace `i` with the corresponding link index.

#### Example

Example usage of environment variables shown below. This provides the _same_
results as the example toml configuration file shown [above](#config-file).

```bash
env PICATCH_ORIGINAL_PHOTOS_DIR=./photos \
    PICATCH_RESIZED_PHOTOS_DIR=./photos_resized \
    PICATCH_PUBLIC.LINKS[0].TEXT=Picatch \
    PICATCH_PUBLIC.LINKS[0].URL=https://github.com/drklee3/picatch \
    PICATCH_PUBLIC.LINKS[1].TEXT=GitHub \
    PICATCH_PUBLIC.LINKS[1].URL=https://github.com/drklee3/ \
    picatch
```

## Compiling From Source

### Requirements

#### Backend

-   [Rust](https://www.rust-lang.org/tools/install) version 1.40.0+

#### Frontend

-   Node.js
-   Yarn

### Building

#### Frontend

```bash
cd web

# Build frontend files
yarn && yarn build
```

If you want to only build the backend yourself (if you don't have Node.js
installed and don't want to install it or something), you can also download the
built frontend files specifically from the [GitHub Actions artifacts][github-workflow-ci].

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
./target/release/picatch
```

## License

MIT.

[actix-web]: https://github.com/actix/actix-web
[demo-master]: https://master.dlee.photo
[demo-master-img]: https://img.shields.io/website?down_color=red&down_message=offline&label=demo%20%28master%29&up_color=blue&up_message=online&url=https%3A%2F%2Fmaster.dlee.photo
[demo-stable]: https://dlee.photo
[demo-stable-img]: https://img.shields.io/website?down_color=red&down_message=offline&label=demo%20%28stable%29&up_color=blue&up_message=online&url=https%3A%2F%2Fdlee.photo
[docker-hub]: https://hub.docker.com/repository/docker/drklee3/picatch
[docker-hub-img]: https://img.shields.io/docker/image-size/drklee3/picatch/master?style=flat
[github-workflow-ci]: https://github.com/drklee3/picatch/actions?query=workflow%3ACI
[github-workflow-docker-img]: https://img.shields.io/github/workflow/status/drklee3/picatch/Docker?style=flat
[github-workflow-docker]: https://github.com/drklee3/picatch/actions?query=workflow%3ADocker
[github-package-auth]: https://help.github.com/en/packages/using-github-packages-with-your-projects-ecosystem/configuring-docker-for-use-with-github-packages#authenticating-to-github-packages
[github-packages]: https://github.com/drklee3/picatch/packages
[image]: https://github.com/image-rs/image

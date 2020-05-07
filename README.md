# dphoto

uhh wip + need a new name 🤔

Minimal photo gallery based on directory structure without a database.

## Requirements

* Rust version 1.39+ (for async/await)

## Building

```bash
# Install Rust -- https://www.rust-lang.org/tools/install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone this repository
git clone git@github.com:drklee3/dphoto.git

# Or clone with HTTPS
git clone https://github.com/drklee3/dphoto.git

cd dphoto

# Build for release
cargo build --release
```

## Running

```bash
cargo run --release
```

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

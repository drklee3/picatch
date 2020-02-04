# dphoto

uhh wip + need a new name 🤔

## Requirements

* Rust version 1.39+ (for async/await)
* PostgreSQL

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

## Configuration

Configuration for the database is stored in a `config.toml` file.  This file
should be created in the project root directory.  An example can be found in
[`config.example.toml`](https://github.com/drklee3/dphoto/blob/master/config.example.toml).

```bash
cp config.example.toml config.toml
```

## Running

```bash
cargo run --release
```

## backend

* [Actix web](https://github.com/actix/actix-web) server
* image resizer w/ [image](https://github.com/image-rs/image)

## client

* todo but probably React

## TODO / Features

* [ ] Auth system
  * [ ] User accounts
  * [ ] Roles (should i bother idk)
  * [ ] Role Permission
  * [ ] Session management
* [ ] Image Handling
  * [ ] Resizing
  * [ ] Caching
* [ ] Uh everything front end
  * [ ] Login
  * [ ] Home page
  * [ ] Albums
  * [ ] Images
  * [ ] Settings

## Development

Use diesel CLI which reads from environment variables.  If using Diesel CLI,
add the database url to a `.env` file.

```bash
cargo install diesel_cli --no-default-features --features postgres

echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
```

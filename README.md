# dphoto

uhh wip + need a new name 🤔

## Requirements

* Rust version 1.39+ (for async/await)
* PostgreSQL
* `libpq-dev`

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

# Then edit config.toml to match your database configuration
```

An annotated version of the example config is shown below.

```toml
[database]
# Full database url, other fields will be ignored if this is provided
url = "postgres://user:pass@host:port/db"

username = "drklee3"
password = "Hunter2"
host = "localhost"
# Port can be omitted to use the default 5432 port.  This should be used mainly if you're using a port other than 5432.
port = "5432"
name = "db name"
```

## Running

```bash
cargo run --release
```

## backend

* [Actix web](https://github.com/actix/actix-web) server
* image resizer w/ [image](https://github.com/image-rs/image)

## client

* todo but probably React.. and maybe Gatsby?

## TODO / Features

* [ ] Authentication
  * [ ] Registration
  * [ ] Login
  * [ ] Session management
  * [ ] Roles (should i bother idk)
  * [ ] Role Permissions
* [ ] Images
  * [ ] Resizing
  * [ ] Caching
* [ ] Albums
  * [ ] Permissions
  * [ ] Visibility
  * [ ] Password
  * [ ] Multiple albums per image
* [ ] Uh front end
  * [ ] Login / Register
  * [ ] Home
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

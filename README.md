# dphoto

uhh wip + need a new name 🤔

requires Rust version 1.39+ (uses async/await yay)

## backend

* [Actix web](https://github.com/actix/actix-web) server
* image resizer w/ [image](https://github.com/image-rs/image)

## client

* todo but probably React

## Development

Use diesel CLI which reads from environment variables.  If using Diesel CLI,
add the database url to a `.env` file.

```bash
cargo install diesel_cli --no-default-features --features postgres

echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
```

Luten
=====

Web app for managing excercise sessions for university courses. ToDo: write a better summary.

## Running

### Prerequisites

In order to build and launch Luten, you need to install a number of tools:

- **A nightly version of the Rust compiler**. You should use `rustup` to
  manage your compiler toolchains. This application should work with the
  newest nightly, but a version that is guaranteed to work can be found in
  `.travis.yml`. You want to set an override via `rustup` (`$ rustup override
  set nightly` in the `luten` directory).
- **A PostgreSQL database**. To install the DBS using Ubuntu, you can execute
  `$ sudo apt install postgresql libpg-dev`. You probably also want to
  `install pgadmin3`.
- **Diesel CLI tool**. Install via `$ cargo install diesel_cli
  --no-default-features --features "postgres"`.
- **SASS compiler**. Using Ubuntu, you can execute `$ sudo apt install
  rubygems && gem install sass`.

After those are installed, you have to execute all database migrations. To do
that, you first have to tell Diesel the path to your database. Create a file
`.env` in the `luten/` folder and fill it with:

```
DATABASE_URL=postgres://username:password@localhost/database_name
```

Afterwards execute `$ diesel setup`. Your system should now be ready.

### Building & Running

Simply execute `$ cargo build`. To run the server, execute `$ cargo run --bin
start_server`.

---

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

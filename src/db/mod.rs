use diesel::pg::PgConnection;
use dotenv::dotenv;
use r2d2::{self, Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;

use std::env;

use errors::*;

pub mod schema;


/// A database connection pool.
///
/// Is managed as `State` in the Rocket instance. Can be retrieved simply by
/// add a `db: State<Db>` parameter to your handler. Afterwards you want to
/// call `.conn()` and pass it to diesel.
pub struct Db {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl Db {
    /// Opens a database connection with the `DATABASE_URL` given as
    /// environment variable. The `.env` is used to load environment variables,
    /// too. If no env variable `DATABASE_URL` is set, this function panics.
    pub fn open_connection() -> Self {
        // Load DATABASE_URL from `.env` if present.
        dotenv().ok();

        // Create a database connection pool.
        // TODO: Maybe tweak r2d2 config if necessary
        // TODO: Maybe install another error handler for the pool
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        let manager = ConnectionManager::new(database_url);

        let config = r2d2::Config::default();

        let pool = Pool::new(config, manager)
            .expect("Failed to create pool.");

        Self { pool }
    }

    /// Returns a DB connection that can be used with diesel.
    pub fn conn(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>> {
        Ok(
            self.pool.get()
                .chain_err(|| "Timeout while obtaining a connection to the database")?
        )
    }
}

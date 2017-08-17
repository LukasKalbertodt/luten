use diesel::pg::PgConnection;
use dotenv::dotenv;
use r2d2::{self, Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;

use std::env;

pub mod schema;


pub struct Db {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl Db {
    pub fn open_connection() -> Self {
        // Load DATABASE_URL from `.env` if present.
        dotenv().ok();

        // Create a database connection pool.
        // TODO: Maybe tweak r2d2 config if necessary
        // TODO: Maybe install another error handler for the pool
        let config = r2d2::Config::default();
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        let manager = ConnectionManager::new(database_url);
        let pool = Pool::new(config, manager).expect("Failed to create pool.");

        Self { pool }
    }

    /// Returns a DB connection.
    pub fn conn(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool.get().unwrap()
    }
}

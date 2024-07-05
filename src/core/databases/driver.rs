//! Database driver factory.
//!
//! See [`databases::driver::build`](crate::core::databases::driver::build)
//! function for more information.
use serde::{Deserialize, Serialize};

use super::error::Error;
use super::mysql::Mysql;
use super::sqlite::Sqlite;
use super::{Builder, Database};

/// The database management system used by the tracker.
///
/// Refer to:
///
/// - [Torrust Tracker Configuration](https://docs.rs/torrust-tracker-configuration).
/// - [Torrust Tracker](https://docs.rs/torrust-tracker).
///
/// For more information about persistence.
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, derive_more::Display, Clone)]
pub enum DatabaseDriver {
    /// The Sqlite3 database driver.
    Sqlite3,
    /// The `MySQL` database driver.
    MySQL,
}

/// It builds a new database driver.
///
/// Example for `SQLite3`:
///
/// ```rust,no_run
/// use torrust_tracker::core::databases;
/// use torrust_tracker::core::databases::driver::DatabaseDriver;
///
/// let db_driver = DatabaseDriver::Sqlite3;
/// let db_path = "./storage/tracker/lib/database/sqlite3.db".to_string();
/// let database = databases::driver::build(&db_driver, &db_path);
/// ```
///
/// Example for `MySQL`:
///
/// ```rust,no_run
/// use torrust_tracker::core::databases;
/// use torrust_tracker::core::databases::driver::DatabaseDriver;
///
/// let db_driver = DatabaseDriver::MySQL;
/// let db_path = "mysql://db_user:db_user_secret_password@mysql:3306/torrust_tracker".to_string();
/// let database = databases::driver::build(&db_driver, &db_path);
/// ```
///
/// Refer to the [configuration documentation](https://docs.rs/torrust-tracker-configuration)
/// for more information about the database configuration.
///
/// > **WARNING**: The driver instantiation runs database migrations.
///
/// # Errors
///
/// This function will return an error if unable to connect to the database.
///
/// # Panics
///
/// This function will panic if unable to create database tables.
pub fn build(driver: &DatabaseDriver, db_path: &str) -> Result<Box<dyn Database>, Error> {
    let database = match driver {
        DatabaseDriver::Sqlite3 => Builder::<Sqlite>::build(db_path),
        DatabaseDriver::MySQL => Builder::<Mysql>::build(db_path),
    }?;

    database.create_database_tables().expect("Could not create database tables.");

    Ok(database)
}

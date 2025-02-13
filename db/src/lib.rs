//!
//! Diesel powered ORM management library for MuOxi
//! uses postgres
//!

#[macro_use]
extern crate diesel;

// pub mod accounts;
pub mod cache;
pub mod cache_structures;
pub mod schema;
pub mod structures;
pub mod utils;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use std::env;

/// Main database handler struct
pub struct DatabaseHandler {
    /// acutal connection to postgres database
    pub handle: PgConnection,
    /// handle to the Accounts table
    pub accounts: structures::account::AccountHandler,

    /// handle to the Characters table
    pub characters: structures::character::CharacterHandler,
}

impl DatabaseHandler {
    /// creates a new instance of the handler and defaults to
    /// postgres url: `postgres://muoxi:muoxi@localhost/muoxi`
    /// When setting up postgresql create user name `muoxi` and password `muoxi`
    /// and/or set the DATABASE_URL environment variable to an appropriate
    /// and valid url. This function will panic if it can't connect to a database.
    pub fn connect() -> Self {
        let url = env::var("DATABASE_URL").unwrap_or("postgres://muoxi:muoxi@localhost/muoxi".to_string());
        let conn = PgConnection::establish(&url).expect("Couldn't create handle to database");
        Self {
            handle: conn,
            // clients: clients::ClientHandler {},
            accounts: structures::account::AccountHandler,
            characters: structures::character::CharacterHandler,
        }
    }
}

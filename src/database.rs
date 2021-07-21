/*
 * This file contains a database abstraction layer to simplify interaction
 * with the underlaying SQLite database from elsewhere in the code
 */

use rusqlite::{Connection, Result, params};

use crate::debtrecord::DebtRecord;

pub struct Database {
    conn: Option<rusqlite::Connection>
}

impl Database {
    pub fn new() -> Database {
        return Database { conn: None };
    }

    /// Initializes the database and creates its tables
    pub fn initialize(&mut self) -> Result<()> {
        self.conn = Some(Connection::open("database.db")?);
        self.create_tables()?;

        Ok(())
    }

    /// Creates all database tables if they do not already exist
    fn create_tables(&mut self) -> Result<()> {
        let scheme = "
            CREATE TABLE IF NOT EXISTS debt (
                id INTEGER PRIMARY KEY,
                debtor TEXT NOT NULL,
                creditor TEXT NOT NULL,
                amount INTEGER NOT NULL DEFAULT '0',
                description TEXT DEFAULT 'N/A'
            )
        ";

        self.get_dbconn().execute(scheme, [])?;

        Ok(())
    }

    /// Drops all tables in the database
    fn drop_tables(&mut self) -> Result<()> {
        let query = "
            DROP TABLE debt
        ";

        self.get_dbconn().execute(query, [])?;

        return Ok(());
    }

    /// Returns the last INSERTed row
    fn get_last_entry(&mut self) -> Result<DebtRecord> {
        let query = "
        SELECT * FROM debt WHERE id = (SELECT MAX(id) FROM debt)
        ";

        let mut statement = self.get_dbconn().prepare(query)?;
        let mut iter = statement.query_map([], |row| {
            Ok(DebtRecord {
                id: row.get(0)?,
                debtor: row.get(1)?,
                creditor: row.get(2)?,
                amount: row.get(3)?,
                description: row.get(4)?
            })
        })?;

        let first = iter.next().unwrap()?;
        return Ok(first);
    }

    /// Adds a new debt entry to the database using the specified parameters
    pub fn add_entry(&mut self, creditor: String, debtor: String, amount: i64, description: String) -> Result<DebtRecord> {
        let query = "
            INSERT INTO debt (debtor, creditor, amount, description) VALUES (?1, ?2, ?3, ?4)
        ";

        self.get_dbconn().execute(query, params![debtor, creditor, amount, description])?;

        return Ok(self.get_last_entry()?);
    }

    /// Deletes a single debt entry from the database via its index
    pub fn remove_entry(&mut self, index: i64) -> Result<DebtRecord> {
        let entry = self.get_entry(index)?;

        let query = "
            DELETE FROM debt WHERE id = ?1
        ";

        self.get_dbconn().execute(query, params![index])?;

        return Ok(entry);
    }

    /// Drops and then re-creates the debt database table
    pub fn reset_database(&mut self) -> Result<()> {
        self.drop_tables()?;
        self.create_tables()?;

        Ok(())
    }

    /// Returns all debt entries in the database
    pub fn get_entries(&mut self) -> Result<Vec<DebtRecord>> {
        let query = "
            SELECT id, debtor, creditor, amount, description FROM debt  
        ";

        let mut statement = self.get_dbconn().prepare(query)?;
        let iter = statement.query_map([], |row| {
            Ok(DebtRecord {
                id: row.get(0)?,
                debtor: row.get(1)?,
                creditor: row.get(2)?,
                amount: row.get(3)?,
                description: row.get(4)?
            })
        })?;

        return iter.collect();
    }

    /// Returns a single database debt entry via its index
    pub fn get_entry(&mut self, index: i64) -> Result<DebtRecord> {
        let query = "
            SELECT id, debtor, creditor, amount, description FROM debt WHERE id = ?1
        ";

        let mut statement = self.get_dbconn().prepare(query)?;
        let mut entry = statement.query_map(params![index], |row| {
            Ok(DebtRecord {
                id: row.get(0)?,
                debtor: row.get(1)?,
                creditor: row.get(2)?,
                amount: row.get(3)?,
                description: row.get(4)?
            })
        })?;

        let first = entry.next().unwrap()?;
        return Ok(first);
    }

    /// Returns the underlaying SQLite database connection object,
    /// useful if you want to interact directly with the database.
    pub fn get_dbconn(&mut self) -> &rusqlite::Connection {
        return self.conn.as_ref().unwrap()
    }
}
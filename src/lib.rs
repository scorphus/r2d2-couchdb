//! [CouchDB](../chill) support for the [`r2d2`](../r2d2) connection pool.
#![warn(missing_docs)]
#![doc(html_root_url="https://scorphus.github.io/r2d2-couchdb")]
extern crate chill;
extern crate r2d2;

use chill::{Client, IntoUrl};

use std::error;
use std::error::Error as _StdError;
use std::fmt;

/// A unified enum of errors returned by chill::Client
#[derive(Debug)]
pub enum Error {
    /// A chill::Error
    Other(chill::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}: {}", self.description(), self.cause().unwrap())
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Other(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Other(ref err) => err.cause()
        }
    }
}

/// An `r2d2::ManageConnection` for
//`[`chill::Client`](../chill/struct.Client.html)`s.
///
/// ## Example
///
/// ```rust
/// extern crate r2d2;
/// extern crate r2d2_couchdb;
/// extern crate serde_json;
///
/// use r2d2_couchdb::{CouchdbConnectionManager};
///
/// use std::thread;
///
/// fn main() {
///     let config = r2d2::Config::default();
///     let manager = CouchdbConnectionManager::new("http://localhost:5984/").unwrap();
///     let pool = r2d2::Pool::new(config, manager).unwrap();
///
///     let mut handles = vec![];
///
///     for i in 0..20 {
///         let pool = pool.clone();
///         handles.push(thread::spawn(move || {
///             let content = serde_json::builder::ObjectBuilder::new()
///                 .insert("foo", i)
///                 .unwrap();
///             println!("Sending {}", &content);
///             let conn = pool.get().unwrap();
///             conn.create_document("/test", &content).run().unwrap();
///         }));
///     }
///
///     for handle in handles {
///         handle.join().unwrap()
///     }
/// }
/// ```
#[derive(Debug)]
pub struct CouchdbConnectionManager {
    server_url: String,
}

impl CouchdbConnectionManager {
    /// Creates a new `CouchdbConnectionManager`.
    pub fn new(server_url: &str)
            -> Result<CouchdbConnectionManager, chill::Error> {
        let _ = try!(server_url.into_url());
        Ok(CouchdbConnectionManager {
            server_url: server_url.to_owned(),
        })
    }
}

impl r2d2::ManageConnection for CouchdbConnectionManager {
    type Connection = Client;
    type Error = Error;

    fn connect(&self) -> Result<Client, Error> {
        Client::new(&self.server_url).map_err(|err| Error::Other(err))
    }

    fn is_valid(&self, _conn: &mut Client) -> Result<(), Error> {
        Ok(())
    }

    fn has_broken(&self, _conn: &mut Client) -> bool {
        false
    }
}

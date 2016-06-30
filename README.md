# r2d2-couchdb [![Build Status][travis-badge]][travis-link] [![MIT License][mit-license-badge]](LICENSE-MIT) [![Apache 2.0][apache-2.0-badge]](LICENSE-APACHE)

[CouchDB] support library for the [r2d2](https://github.com/scorphus/r2d2) connection pool. Read the [documentation].


# Example

```rust
extern crate r2d2;
extern crate r2d2_couchdb;
extern crate serde_json;

use r2d2_couchdb::{CouchdbConnectionManager};

use std::thread;

fn main() {
    let config = r2d2::Config::default();
    let manager = CouchdbConnectionManager::new("http://localhost:5984/").unwrap();
    let pool = r2d2::Pool::new(config, manager).unwrap();

    let mut handles = vec![];

    for i in 0..20 {
        let pool = pool.clone();
        handles.push(thread::spawn(move || {
            let content = serde_json::builder::ObjectBuilder::new()
                .insert("foo", i)
                .unwrap();
            println!("Sending {}", &content);
            let conn = pool.get().unwrap();
            conn.create_document("/test", &content).run().unwrap();
        }));
    }

    for handle in handles {
        handle.join().unwrap()
    }
}
```


[travis-badge]:       https://img.shields.io/travis/scorphus/r2d2-couchdb.svg
[travis-link]:        https://travis-ci.org/scorphus/r2d2-couchdb
[mit-license-badge]:  https://img.shields.io/badge/license-MIT-007EC7.svg
[apache-2.0-badge]:   https://img.shields.io/badge/license-Apache%202.0-007EC7.svg
[CouchDB]:            https://github.com/chill-rs/chill
[documentation]:      https://scorphus.github.io/r2d2-couchdb/

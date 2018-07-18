extern crate futures;
extern crate tokio_postgres;

use futures::Future;
use tokio_postgres::{Connection, TlsMode};

fn find_users() {
    let mut l = Core::new().unwrap();
    let done = Connection::connect("postgresql://postgres@localhost",
                                   TlsMode::None,
                                   &l.handle());
}
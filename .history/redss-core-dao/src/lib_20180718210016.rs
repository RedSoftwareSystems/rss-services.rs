extern crate futures;
extern crate tokio;
extern crate tokio_postgres;

use tokio::prelude::*;
use futures::Future;
use tokio_postgres::{TlsMode};
use proto::connect::ConnectFuture;

fn find_users() {
    let handshkae = tokio_postgres::connect("postgresql://postgres@localhost".parse().unwrap(),
                                   TlsMode::None);
}
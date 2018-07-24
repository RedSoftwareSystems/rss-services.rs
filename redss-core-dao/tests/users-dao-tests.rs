
extern crate tokio;
extern crate redss_core_dao;
extern crate tokio_postgres;
extern crate serde_json;

extern crate futures;

use tokio_postgres::{TlsMode};
use tokio_postgres::error::SqlState;

use redss_core_dao::users;
use redss_core_dao::models;

use tokio::prelude::*;
use tokio::runtime::current_thread::Runtime;

use serde_json::{Value};

#[test]
fn test_find_by_email() {
    let mut runtime = Runtime::new().unwrap();

    let handshake = tokio_postgres::connect(
        "postgres://pgactix:pgactix@localhost:5433".parse().unwrap(),
        TlsMode::None,
    );

    let (client, connection) = runtime.block_on(handshake).unwrap();
    let connection = connection.map_err(|e| panic!("{}", e));
    runtime.handle().spawn(connection).unwrap();

    let user1: Option<Value> = runtime.block_on(users::get_user_by_email(client, "kiuma72@gmail.com")).unwrap();
    assert!(user1.is_some());

    //let user2: Option<Value> = runtime.block_on(users::get_user_by_email(client, "kiuma72@gmail.com")).unwrap();
    //assert!(user2.is_some());
    
    

}
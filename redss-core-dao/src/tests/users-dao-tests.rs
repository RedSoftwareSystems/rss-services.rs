extern crate redss_core_dao;

extern crate tokio;
extern crate redss_core_dao;
extern crate tokio_postgres;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate futures;

use tokio_postgres::{TlsMode,Handshake};

use redss_core_dao::users;
use redss_core_dao::models;

#[test]
fn test_find_user() {
    let user = users::find_user_email("email");
    assert!(user.is_some());
}
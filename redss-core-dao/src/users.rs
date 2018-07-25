// -- users.rs -- //

use std::vec::Vec;
use tokio_postgres::{Client, Row};
use tokio_postgres::error::Error;

use futures::Future;
use tokio::prelude::*;

use serde_json::{Value};
//use models::User;

fn find_by_email(mut client: Client, email: &'static str) -> impl Future<Item = (Option<Row>, Client), Error = Error> + 'static {
    let prepared_s = client.prepare("SELECT data FROM users WHERE data->>'email' = $1");

    let result = prepared_s.and_then(move |statement|
        client.query(&statement, &[&email])
            .collect()
            .map(|mut rows: Vec<Row>| {
                (rows.pop(), client)
            })
    );
    result
}


pub fn get_user_by_email(client: Client, email: &'static str) -> impl Future<Item = (Option<Value>, Client), Error = Error> + 'static {
    find_by_email(client, email).map(|(row, client)| {
        
        match row {
            Some(row) => {
                let data: Value = row.get(0);
                (Some(data), client)
            },
            _ => {                
                (None, client)
            }
        }
    })
}

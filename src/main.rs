use std::{env, sync::Arc};

use albion_api::fetcher::fetch_events;
use error::Error;
use schema::{schema_create, schema_drop};
use socket::socket_handler;
use tokio::sync::mpsc;
use tokio_postgres::{connect, NoTls};
use web::rocket;

mod schema;
mod albion_api;
mod error;
mod socket;
mod web;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (client, connection) = connect("host=localhost user=stk dbname=killarchive", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let (sx, mut rx) = mpsc::channel(1);

    for arg in env::args() {
        match arg.as_str() {
            "create" => schema_create(&client).await.unwrap_or_else(
                |err| println!("Failed to apply schema: {err:?}")
            ),
            "drop" => schema_drop(&client).await.unwrap_or_else(
                |err| println!("Failed to drop schema: {err:?}")
            ),
            _ => ()
        }
    }

    if env::args().len() <= 1 {
        fetch_events(&client).await?;
    }

    let client = Arc::new(client);

    tokio::spawn(socket_handler(client.clone(), sx));

    tokio::spawn(rocket(client));

    rx.recv().await.unwrap_or(Err(Error::MpscRecvError))
}

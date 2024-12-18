use std::{env, sync::Arc};

use albion_api::fetcher::schedule;
use env_logger::Env;
use error::Error;
use schema::{schema_create, schema_drop};
use socket::socket_handler;
use tokio::sync::mpsc;
use tokio_postgres::{connect, NoTls};
use web::launch_web;

mod schema;
mod albion_api;
mod error;
mod socket;
mod web;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let (client, connection) = connect("host=localhost user=stk dbname=killarchive", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let (sx, mut rx) = mpsc::channel(1);

    let client = Arc::new(client);

    tokio::spawn(socket_handler(client.clone(), sx));

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
        tokio::spawn(launch_web(client.clone()));
        tokio::spawn(schedule(client));
        rx.recv().await.unwrap_or(Err(Error::MpscRecvError))
    } else {
        Ok(())
    }

}

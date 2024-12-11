use std::{env::var, fs, sync::Arc};

use tokio::{io::AsyncReadExt, net::{UnixListener, UnixStream}, sync::mpsc};
use tokio_postgres::Client;

use crate::{albion_api::fetcher::fetch_events, error::Error, schema::{schema_create, schema_drop}};

pub async fn socket_handler(client: Arc<Client>, sx: mpsc::Sender<Result<(), Error>>) {
    let listener = match connect().await {
        Ok(v) => v,
        Err(err) => {
            let _ = sx.send(Err(err));
            return
        }
    };
    println!("... Success");

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                println!("Established new socket connection");
                tokio::spawn(client_handler(stream, client.clone(), sx.clone()));
            }
            Err(err) => {
                println!("Error trying to accept a socket connection: {err:?}");
            }
        }
    }
}

async fn connect() -> Result<UnixListener, Error> {
    let path = var("XDG_RUNTIME_DIR")
        .unwrap_or_else(|_|
            var("UID").map_or_else(
                |_| {
                    eprintln!("Neither $XDG_RUNTIME_DIR nor $UID set, fallback to /tmp");
                    "/tmp".to_string()
                },
                |uid| format!("/run/user/{uid}")
            )
        );
    let path = format!("{path}/killarchive");
    let _ = fs::remove_file(&path);
    println!("Attempting to create a UnixSocket at {path} ...");

    Ok(UnixListener::bind(&path)?)
}

async fn client_handler(mut socket: UnixStream, client: Arc<Client>, sx: mpsc::Sender<Result<(), Error>>) -> Result<(), Error> {
    loop {
        let mut bytes = vec![0; 20];
        let n = socket.read(&mut bytes).await?;
        if n == 0 {
            return Ok(());
        }
        let message = String::from_utf8_lossy(&bytes[..n]).to_string();
        println!("Received command: {}", message.trim());

        match message.to_lowercase().trim() {
            "exit" | "quit" | "close" | "shutdown" => {
                println!("Exiting...");
                sx.send(Ok(())).await.expect("Exit channel closed");
                return Ok(());
            }
            "help" | "h" => {
                println!("Request for help was denied");
            }
            "fetch" => {
                fetch_events(&client).await?;
            }
            "create" => schema_create(&client).await.unwrap_or_else(
                |err| println!("Failed to apply schema: {err:?}")
            ),
            "drop" => schema_drop(&client).await.unwrap_or_else(
                |err| println!("Failed to drop schema: {err:?}")
            ),
            _ => {}
        }
    }
}

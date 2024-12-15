use std::sync::Arc;

use actix_web::{get, http::header::ContentType, middleware::Logger, web::Data, App, HttpResponse, HttpServer};
use players::players_index;
use serde::Deserialize;
use tera::Tera;
use tokio_postgres::Client;

use crate::error::Error;

mod players;

pub async fn launch_web(client: Arc<Client>) -> Result<(), Error> {
    let tera = Tera::new("src/web/tera/**/*.html")?;
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("\"%r\" Status: %s\nHEADERS: %{FOO}i\n"))
            .app_data(Data::new(client.clone()))
            .app_data(Data::new(tera.clone()))
            .service(root)
            .service(players_index)
    })
    .bind(("127.0.0.1", 9000)).unwrap()
    .run()
    .await
    .map_err(|err| err.into())
}

#[get("/")]
async fn root() -> HttpResponse {
    let response = format!("<h1>killarchive-rs</h1><ul>{}</ul>",
        ["Players"]
        .into_iter()
        .map(|item|
            format!("<li><a href=\"{}\">{item}</a></li>",
                item.to_lowercase()
            )
        ).collect::<Vec<String>>()
        .concat()
    );
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(response)
}

#[derive(Deserialize)]
struct Params {
    offset: Option<i32>,
}

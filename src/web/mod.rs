use std::sync::Arc;

use actix_web::{get, http::header::ContentType, middleware::Logger, web::Data, App, HttpResponse, HttpServer};
use players::{player_show, players_index};
use serde::Deserialize;
use tera::{Context, Tera};
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
            .service(favicon)
            .service(player_show)
            .service(players_index)
    })
    .bind(("127.0.0.1", 9000)).unwrap()
    .run()
    .await
    .map_err(|err| err.into())
}

#[get("/")]
async fn root(client: Data<Arc<Client>>, tera: Data<Tera>) -> Result<HttpResponse, Error> {
    let last_hour_event_count = client
        .query("SELECT COUNT(*) FROM cached_events WHERE timestamp > NOW() - INTERVAL '1 hour'", &[])
        .await?
        .into_iter()
        .map(|row| row.get(0))
        .collect::<Vec<i64>>();

    let mut context = Context::new();
    context.insert("event_count", &last_hour_event_count[0]);

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(tera.render("root.html", &context)?))
}

#[get("/favicon.ico")]
async fn favicon() -> HttpResponse {
    HttpResponse::Ok()
        .body(&include_bytes!("../../static/favicon.ico")[..])
}

#[derive(Deserialize)]
struct Params {
    offset: Option<i32>,
    order_by: Option<String>
}

impl Params {
    fn get_order(&self) -> String {
        match self.order_by.as_ref().unwrap_or(&"".to_string()).as_str() {
            "kills" | "deaths" | "assists" | "allies" => self.order_by.as_ref().unwrap().clone(),
            _ => "kills".to_string()
        }
    }
    fn get_secundary_order(&self) -> &'static str {
        match self.order_by.as_ref().unwrap_or(&"kills".to_string()).as_str() {
            "deaths" => "kills",
            "assists" => "allies",
            _ => "assists"
        }
    }
}

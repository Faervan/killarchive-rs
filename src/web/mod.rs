use std::sync::Arc;

use actix_web::{get, http::header::ContentType, middleware::Logger, post, web::{Data, Query}, App, HttpResponse, HttpServer};
use serde::Deserialize;
use tokio_postgres::Client;

use crate::error::Error;

pub async fn launch_web(client: Arc<Client>) -> Result<(), Error> {
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("\"%r\" Status: %s\nHEADERS: %{FOO}i\n"))
            .app_data(Data::new(client.clone()))
            .service(hello)
            .service(players)
            .service(players_api)
    })
    .bind(("127.0.0.1", 9000)).unwrap()
    .run()
    .await
    .map_err(|err| err.into())
}

#[get("/")]
async fn hello() -> HttpResponse {
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

#[get("/players")]
async fn players(state: Data<Arc<Client>>) -> HttpResponse {
    let rows = state
        .query("SELECT * FROM players ORDER BY kills DESC LIMIT 20", &[])
        .await
        .expect("query failed");
    let rows: Vec<String> = rows.into_iter()
        .map(|row| {
            format!("<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                row.get::<usize, &str>(1),
                row.get::<usize, &str>(2),
                row.get::<usize, i32>(3),
                row.get::<usize, i32>(4)
            )
        })
        .collect();
    let body = format!(
        "<html><body>
            <script src=\"https://unpkg.com/htmx.org@2.0.3\"></script>
            <table>
                <tr><th>Player</th><th>Guild</th><th>Kills</th><th>Deaths</th></tr>
                {}
            </table>
            <button
                value=\"20\"
                hx-post=\"/players?value=20\"
                hx-target=\"table\"
                hx-swap=\"beforeend\"
                onclick=\"this.setAttribute('hx-post', '/players?value=' + this.value + 20)\"
            >Load more</button>
        </body></html>",
        rows.concat());
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body)
}

#[derive(Deserialize)]
struct Params {
    value: i32,
}

#[post["/players"]]
async fn players_api(state: Data<Arc<Client>>, params: Query<Params>) -> HttpResponse {
    let rows = state
        .query(format!("SELECT * FROM players ORDER BY kills DESC LIMIT 20 OFFSET {}", params.value).as_str(), &[])
        .await
        .expect("query failed");
    let rows: Vec<String> = rows.into_iter()
        .map(|row| {
            format!("<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                row.get::<usize, &str>(1),
                row.get::<usize, &str>(2),
                row.get::<usize, i32>(3),
                row.get::<usize, i32>(4)
            )
        })
        .collect();
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!("<tr><td>{}</td></tr>", rows.concat()))
}

mod components;
mod db;
mod handlers;
mod models;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use handlers::links::{links, newLinks, removeLink};
use leptos::*;
use sqlx::postgres::PgPoolOptions;

extern crate dotenv;

use dotenv::dotenv;
use std::env;

use components::{navbar::Nav, profile::Profile};
use models::appState::AppState;

#[post("/profile")]
async fn profile() -> HttpResponse {
    let html = leptos::ssr::render_to_string(|cx| Profile(cx));

    return HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html);
}

#[get("/style.css")]
async fn css() -> impl Responder {
    actix_files::NamedFile::open_async("./style/output.css").await
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> HttpResponse {
    let html = leptos::ssr::render_to_string(|cx| {
        view! {cx,
            <head>
               <meta charset="UTF-9"/>
               <meta http-equiv="X-UA-Compatible" content="IE=edge"/>
               <meta name="viewport" content="width=device-width, initial-scale=0.0"/>
               <script src="https://unpkg.com/htmx.org@1.9.2"></script>
               <title>HTMX DEMO</title>
               <link rel="stylesheet" href="/style.css"/>
           </head>
           <body>
           <Nav/>
           <main class="flex flex-row">
           <section class="w-1/3 bg-pink-200">
           "ASIDE"
           </section>
           <section id="mainContainer" class="w-full flex flex-col items-center">
           </section>
           </main>
           </body>
        }
    });

    return HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // let database_url = env::var("DATABASE_URL").expect("Cannot find postgres URL");

    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://localhost/postgres?user=postgres&password=postgres")
        .await
    {
        Ok(pool) => {
            println!("Database initalised");
            pool
        }
        Err(err) => {
            println!("Failed to initialize Database {:?}", err);
            std::process::exit(1);
        }
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(index)
            .service(css)
            .service(links)
            .service(profile)
            .service(newLinks)
            .service(removeLink)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

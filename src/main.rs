mod components;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use leptos::*;

use components::{customizeLinks::CustomLinks, navbar::Nav, profile::Profile};

#[post("/links")]
async fn links() -> HttpResponse {
    let html = leptos::ssr::render_to_string(|cx| CustomLinks(cx));

    return HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html);
}

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
async fn index() -> HttpResponse {
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
                <CustomLinks/>
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
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(css)
            .service(links)
            .service(profile)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

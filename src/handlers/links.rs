use actix_web::{
    get, post,
    web::{self},
    HttpResponse,
};
use leptos::*;
use serde::{Deserialize, Serialize};
use sqlx::Row;

use crate::{
    components::customizeLinks::CustomLinks,
    models::{appState::AppState, link::Link, platform::Platform},
};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NLink {
    pub data: String,
    // pub platform: Platform,
    // pub userid: String,
}
#[get("/links")]
pub async fn links(data: web::Data<AppState>) -> HttpResponse {
    let rows = sqlx::query("SELECT * FROM links")
        .fetch_all(&data.db)
        .await
        .expect("Failed to fetch links");

    let res = rows
        .into_iter()
        .map(|row| Link {
            linkid: row.get("linkid"),
            val: row.get("val"),
            userid: row.get("userid"),
            platform: Platform::GITHUB,
        })
        .collect::<Vec<Link>>();

    let html = leptos::ssr::render_to_string(|cx| {
        view! {cx,
        <CustomLinks links={res}/>}
    });

    return HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html);
}

#[post("/newLink")]
pub async fn newLinks(req: web::Form<NLink>, data: web::Data<AppState>) -> HttpResponse {
    let oldList: Vec<Link> = serde_json::from_str(&req.data.replace("'", "\""))
        .expect("failed to deserialize the old list");

    let mut new_list = vec![Link::new(None, None, "userID"); 1];

    new_list.extend(oldList.into_iter());

    let html = leptos::ssr::render_to_string(|cx| {
        view! {cx,
        <CustomLinks links={new_list}/>}
    });

    return HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html);
}

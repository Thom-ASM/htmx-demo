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

pub struct New_Link_Body {
    pub count: usize,
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
pub async fn newLinks(req: web::Form<New_Link_Body>, data: web::Data<AppState>) -> HttpResponse {
    let rows = sqlx::query("SELECT * FROM links")
        .fetch_all(&data.db)
        .await
        .expect("Failed to fetch links");

    let mut res = rows
        .into_iter()
        .map(|row| Link {
            linkid: row.get("linkid"),
            val: row.get("val"),
            userid: row.get("userid"),
            platform: Platform::GITHUB,
        })
        .collect::<Vec<Link>>();

    let new_items = req.count - res.len() + 1;

    res.extend(vec![Link::new(None, None, "userID"); new_items]);

    let html = leptos::ssr::render_to_string(|cx| {
        view! {cx,
        <CustomLinks links={res}/>}
    });

    return HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html);
}

use actix_web::{
    get, post,
    web::{self},
    HttpResponse,
};
use leptos::*;
use sqlx::Row;

use crate::{
    components::customizeLinks::CustomLinks,
    db::{mutations::insert_new_link, queries::get_all_links_for_user},
    models::{appState::AppState, link::Link, platform::Platform},
};
#[get("/links")]
pub async fn links(data: web::Data<AppState>) -> HttpResponse {
    let mut errorMessage: Option<String> = None;

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
            active: row.get("active"),
        })
        .collect::<Vec<Link>>();

    let html = leptos::ssr::render_to_string(|cx| {
        view! {cx,
        <CustomLinks links={res} errorMessage={errorMessage}/>}
    });

    return HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html);
}

//TODO: this should take a user id
#[post("/newLink")]
pub async fn newLinks(data: web::Data<AppState>) -> HttpResponse {
    let mut errorMessage: Option<String> = None;

    match insert_new_link("abe6a8e7-083f-4439-9573-d5bbf137355f".to_owned())
        .execute(&data.db)
        .await
    {
        Err(_) => errorMessage = Some("Failed to create new field".to_owned()),
        _ => {}
    }

    let rows = get_all_links_for_user("abe6a8e7-083f-4439-9573-d5bbf137355f".to_string())
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
            active: row.get("active"),
        })
        .collect::<Vec<Link>>();

    let html = leptos::ssr::render_to_string(|cx| {
        view! {cx,
        <CustomLinks links={res} errorMessage={errorMessage}/>}
    });

    return HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html);
}

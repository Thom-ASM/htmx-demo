use actix_web::{
    delete, get, post,
    web::{self},
    HttpResponse,
};
use leptos::*;
use serde::Deserialize;
use sqlx::Row;

use crate::{
    components::customizeLinks::CustomLinks,
    db::{
        mutations::{delete_link_by_id, insert_new_link},
        queries::get_all_links_for_user,
    },
    models::{appState::AppState, link::Link, platform::Platform},
};

#[get("/links")]
pub async fn links(data: web::Data<AppState>) -> HttpResponse {
    let mut errorMessage: Option<String> = None;

    let rows =
        match get_all_links_for_user("abe6a8e7-083f-4439-9573-d5bbf137355f".to_string(), &data.db)
            .await
        {
            Ok(rows) => rows,
            Err(e) => {
                errorMessage = Some(e);
                vec![]
            }
        };

    let html = leptos::ssr::render_to_string(|cx| {
        view! {cx,
        <CustomLinks links={rows} errorMessage={errorMessage}/>}
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

    let rows =
        match get_all_links_for_user("abe6a8e7-083f-4439-9573-d5bbf137355f".to_string(), &data.db)
            .await
        {
            Ok(rows) => rows,
            Err(e) => {
                errorMessage = Some(e);
                vec![]
            }
        };

    let html = leptos::ssr::render_to_string(|cx| {
        view! {cx,
        <CustomLinks links={rows} errorMessage={errorMessage}/>}
    });

    return HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html);
}

//Remove struct
#[derive(Deserialize, Debug, Clone)]
pub struct RemoveLink {
    pub id: String,
}

#[delete("/removeLink")]
pub async fn removeLink(req: web::Form<RemoveLink>, data: web::Data<AppState>) -> HttpResponse {
    let mut errorMessage: Option<String> = None;

    println!(" deleting {}", req.id);

    match delete_link_by_id(req.id.clone()).execute(&data.db).await {
        Err(e) => {
            errorMessage = {
                println!("{:?}", e);
                Some("Failed to delete field".to_owned())
            }
        }
        _ => {}
    }

    let rows =
        match get_all_links_for_user("abe6a8e7-083f-4439-9573-d5bbf137355f".to_string(), &data.db)
            .await
        {
            Ok(rows) => rows,
            Err(e) => {
                errorMessage = Some(e);
                vec![]
            }
        };

    let html = leptos::ssr::render_to_string(|cx| {
        view! {cx,
        <CustomLinks links={rows} errorMessage={errorMessage}/>}
    });

    return HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html);
}

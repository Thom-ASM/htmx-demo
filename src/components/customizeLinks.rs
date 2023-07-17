use actix_web::web;
use leptos::{ev::formdata, *};
use sqlx::Row;

use crate::models::{appState::AppState, link::Link, platform::Platform};

#[component]
pub fn LinkContainer(
    cx: Scope,
    platform: Platform,
    url: String,
    number: usize,
    id: String,
) -> impl IntoView {
    let opts = [Platform::GITHUB, Platform::LINKEDIN, Platform::YOUTUBE];
    return view! {cx,
       <li class="flex flex-col list-none space-y-5" hx-vals=format!("{{\"test\" : \"{}\" }}",url)>
       <section class="flex flex-row justify-between">
        <h3>"Link #"{number}</h3>
        <button hx-vals=format!("{{\"id\":\"{}\"}}",url)>"Remove"</button>
       </section>
       <section class="flex flex-col space-y-2">
       <label for="platform">"Platform"</label>
       <select class="ring-1 ring-gray-500 rounded-sm py-3" name="platform">
       {opts.map(|opt|{
        return view! {cx,
        <option>
        {opt.to_string()}
        </option>}
       })}
       </select>

       <label for="link">"URL"</label>
       <input class="ring-1 ring-gray-500 rounded-sm py-3" type="text" name="link" value=url placeholder="URL"/>
       </section>


       </li>
    };
}

#[component]
pub fn CustomLinks(cx: Scope, links: Vec<Link>) -> impl IntoView {
    let links_to_render = links
        .iter()
        .enumerate()
        .map(|(idx, link)| {
            return view! {cx,
                <LinkContainer number={idx} platform={link.platform.clone()} url={link.val.to_string()} id={link.linkid.clone()}/>
            };
        })
        .collect::<Vec<View>>();

    let formData = serde_json::to_string(&links)
        .expect("Failed to serialize the form data")
        .replace("\"", "'");

    return view! {cx,
       <section class="flex flex-col space-y-8">
        <h2 class="text-4xl color-gray-600 text-bold">"Customise your links"</h2>
        <p class="text-md color-gray-300">"Add/Edit/Remove links below and then share all your profiles with the world"</p>
        <button  hx-post="/newLink"
                 hx-trigger="click"
                 hx-target="#mainContainer"
                 hx-swap="innerHTML"
                 hx-vals=format!("{{\"data\": \"{}\" }}",formData)
                 id="addLink"
                 class="w-full ring-1 ring-purple-500 text-purple-500 rounded-md py-3 text-md"  >"+Add new link"</button>
        <section id="links-container">

        {links_to_render}

        </section>
       </section>
    };
}

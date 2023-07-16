use leptos::*;

#[component]
pub fn Nav(cx: Scope) -> impl IntoView {
    return view! {cx,
        <nav class="w-full h-1/5 flex justify-between px-32 items-center">
            <div>
            // <img></img>
            <h1 class="text-4xl font-extrabold">"devlinks"</h1>
            </div>
            <div class="flex space-x-12">
                <button hx-get="/links"
                 hx-trigger="click"
                 hx-target="#mainContainer"
                 hx-swap="innerHTML"
                 class="hover:text-purple-500">"links"</button>
                <button

                hx-post="/profile"
                hx-trigger="click"
                hx-target="#mainContainer"
                hx-swap="innerHTML"
                class="hover:text-purple-500">"Profile details"</button>
            </div>
            <button class="ring-2 rounded-md text-bold ring-purple-500 text-purple-500 hover:bg-purple-500 hover:ring-white hover:text-white py-3 px-6 font-bold">"Preview"</button>
        </nav>
    };
}

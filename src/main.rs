#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;

const _STYLE: &str = manganis::mg!(file("public/tailwind.css"));

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

fn App() -> Element {
    rsx! {
        div {
            "data-theme": "garden",
            class: "bg-base-100 text-base-content min-h-screen",
            div {
                class: "max-w-[100vw] px-4 pt-8 pb-16",
                Router::<Route> {}
            }
        }
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { class: "btn btn-outline", to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Link {
            class: "btn btn-primary",
            to: Route::Blog {
                id: count()
            },
            "Go to blog"
        }
        div {
            h1 { "High-Five counter: {count}" }
            button { class: "btn btn-accent", onclick: move |_| count += 1, "Up high!" }
            button { class: "btn btn-accent", onclick: move |_| count -= 1, "Down low!" }
        }
    }
}

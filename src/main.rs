// Build with:
// dioxus build --release --features web
// cargo run --features ssr --release
#![allow(non_snake_case)]
use crate::kalosm::learn::KalosmLearn;
pub use crate::router_blog::BookRoute as BlogRoute;
use crate::{router::BookRoute as KalosmBookRoute, router_floneum::BookRoute, search::SearchModal};
use blog::Blog;
use dioxus::prelude::*;

use home::Home;
use kalosm::{KalosmHeaderFooter, KalosmHome};
use learn::Learn;
use serde::{Deserialize, Serialize};

mod blog;
mod components;
#[cfg(feature = "check_docs")]
mod doc_snippets;
mod footer;
mod header;
mod home;
mod kalosm;
mod learn;
mod plugin;
mod router;
mod router_blog;
mod router_floneum;
mod search;
mod shortcut;
mod structured_generation_visualized;
mod table_of_contents;

#[component]
fn HeaderFooter() -> Element {
    rsx! {
        SearchModal { index: &SEARCH_INDEX }
        header::Header {}
        Outlet::<Route> {}
        footer::Footer {}
    }
}

#[derive(Routable, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(KalosmHeaderFooter)]
        #[nest("/kalosm")]
            #[route("/")]
            KalosmHome {},
            #[layout(KalosmLearn)]
                #[child("/docs")]
                KalosmDocs { child: KalosmBookRoute },
            #[end_layout]
        #[end_nest]
    #[end_layout]
    #[layout(HeaderFooter)]
        #[route("/")]
        Home {},
        #[layout(Learn)]
            #[child("/docs")]
            Docs { child: BookRoute },
        #[end_layout]
        #[layout(Blog)]
            #[child("/blog")]
            Blog { child: BlogRoute },
}

pub(crate) use crate::structured_generation_visualized::{
    DerivingParsers, HtmlStructuredGenerationAcceleratedVisualization,
    StructuredGenerationAcceleratedVisualization, StructuredGenerationVisualization,
    TokenizationVisualization,
};

fn main() {
    #[cfg(feature = "web")]
    {
        wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }

    #[cfg(feature = "prebuild")]
    {
        simple_logger::SimpleLogger::new()
            .with_level(log::LevelFilter::Warn)
            .init()
            .unwrap();

        std::env::remove_var("DIOXUS_ACTIVE");
        std::env::remove_var("CARGO");

        LaunchBuilder::new()
            .with_cfg(dioxus::static_site_generation::Config::new().github_pages())
            .launch(app);
        println!("prebuilt");

        dioxus_search::SearchIndex::<Route>::create(
            "search",
            dioxus_search::BaseDirectoryMapping::new(std::path::PathBuf::from("./docs")).map(
                |route: Route| {
                    let route = route.to_string();
                    if route.contains("kalosm") {
                        return None;
                    }
                    let mut path = std::path::PathBuf::new();
                    for segment in route.split('/') {
                        path.push(segment);
                    }
                    Some(path.join("index.html"))
                },
            ),
        );

        dioxus_search::SearchIndex::<Route>::create(
            "kalosm-search",
            dioxus_search::BaseDirectoryMapping::new(std::path::PathBuf::from("./docs")).map(
                |route: Route| {
                    let route = route.to_string();
                    if !route.contains("kalosm") {
                        return None;
                    }
                    let mut path = std::path::PathBuf::new();
                    for segment in route.split('/') {
                        path.push(segment);
                    }
                    Some(path.join("index.html"))
                },
            ),
        );
    }

    #[cfg(not(feature = "prebuild"))]
    launch(app);
}

fn app() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./public/output.css") }
        document::Link { rel: "stylesheet", href: "https://cdnjs.cloudflare.com/ajax/libs/github-markdown-css/5.1.0/github-markdown-light.min.css" }
        document::Link { rel: "icon", href: asset!("./public/assets/Icon.png"), type: "image/png" }
        Router::<Route> {}
    }
}

static SEARCH_INDEX: dioxus_search::LazySearchIndex<Route> = dioxus_search::load_search_index! {
    "search"
};

static KALOSM_SEARCH_INDEX: dioxus_search::LazySearchIndex<Route> = dioxus_search::load_search_index! {
    "kalosm-search"
};

#[component]
fn CodeBlock(contents: String, name: Option<String>) -> Element {
    let mut copied = use_signal(|| false);
    rsx! {
        div { class: "border overflow-hidden rounded-md border-gray-300 dark:border-gray-700 mb-8",
            div { class: "w-full bg-red flex flex-row justify-between border-b border-gray-300 dark:border-gray-700 py-1 px-2 text-xs items-center bg-gray-100 dark:bg-ideblack",
                div { class: "font-mono",
                    if let Some(path) = name {
                        "src/{path}"
                    }
                }
                button {
                    class: "hover:text-blue-600 flex flex-row items-center gap-1",
                    class: if copied() { "text-green-600" },
                    "onclick": "navigator.clipboard.writeText(this.parentNode.parentNode.lastChild.innerText);",
                    onclick: move |_| copied.set(true),
                    if copied() {
                        "Copied!"
                    }
                    span { Copy {} }
                }
            }
            div { class: "codeblock", dangerous_inner_html: contents }
        }
    }
}

fn Copy() -> Element {
    rsx!(
        svg {
            width: "24",
            height: "24",
            stroke_width: "1.5",
            fill: "none",
            stroke: "currentColor",
            path { d: "M8 16c0 1.886 0 2.828.586 3.414C9.172 20 10.114 20 12 20h4c1.886 0 2.828 0 3.414-.586C20 18.828 20 17.886 20 16v-4c0-1.886 0-2.828-.586-3.414C18.828 8 17.886 8 16 8m-8 8h4c1.886 0 2.828 0 3.414-.586C16 14.828 16 13.886 16 12V8m-8 8c-1.886 0-2.828 0-3.414-.586C4 14.828 4 13.886 4 12V8c0-1.886 0-2.828.586-3.414C5.172 4 6.114 4 8 4h4c1.886 0 2.828 0 3.414.586C16 5.172 16 6.114 16 8" }
        }
    )
}
